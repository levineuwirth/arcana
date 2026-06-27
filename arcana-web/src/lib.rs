//! Game-driving core for the `arcana-web` server, kept separate from the HTTP
//! wiring (`main.rs`) so it can be unit-tested without a socket.
//!
//! [`GameCore`] owns one interactive [`Session`] (human seat P0 vs a snappy bot
//! seat P1) and exposes exactly what the two play endpoints need:
//!
//! * [`GameCore::snapshot`] — drive the session through every bot/trivial
//!   decision (`Session::advance`) and project the resulting human decision (or
//!   the finished game) into a serializable [`StateResponse`]. Backs `GET /state`.
//! * [`GameCore::apply_index`] — apply the human's chosen action by its index
//!   into the most-recently-surfaced legal list, then snapshot again. Backs
//!   `POST /action`. Out-of-range indices and "no decision pending" are returned
//!   as a graceful [`ApplyError`], never a panic.
//!
//! The wire format is [`arcana_core::view::ViewState`] (plus the opponent action
//! log), which serde-serializes to JSON and is what the browser renders.

use std::fmt;

use arcana_ai::information_set::project;
use arcana_ai::search::{MaterialValue, ValueMcPolicy};
use arcana_ai::session::{Seat, Session, Turn};
use arcana_core::actions::Action;
use arcana_core::catalog::{card_info, CardInfo};
use arcana_core::deck::parse_deck_text;
use arcana_core::combat::{
    attacker_options, blocker_options, damage_targets, legal_block_declaration,
    match_attack, match_damage,
    match_ordering, ordering_targets, AttackerDeclaration, BlockerDeclaration, DamageAssignment,
    DefendingEntity,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::objects::ObjectId;
use arcana_core::state::GameState;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameResult;
use arcana_core::types::{CardId, ColorSet, PlayerId};
use arcana_core::view::{build_choice_view, view_state, ViewState};
use serde::{Deserialize, Serialize};

/// The human always sits in seat 0; the bot in seat 1.
pub const HUMAN: PlayerId = 0;
/// Deck seed for the (mirrored) sample decks — fixed so both seats are even.
pub const DECK_SEED: u64 = 7;

/// A resolved deck entry for the deckbuilder: the full card info plus its count.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct DeckEntry {
    pub info: CardInfo,
    pub count: u32,
}

/// The result of importing a deck list: resolved main + sideboard (renderable
/// cards) and the names that didn't resolve against the catalog.
#[derive(Clone, Debug, Serialize, PartialEq)]
pub struct ImportedDeck {
    pub name: String,
    pub main: Vec<DeckEntry>,
    pub sideboard: Vec<DeckEntry>,
    pub unresolved: Vec<(String, u32)>,
}

/// Parse an Arena/MTGO deck list and project each resolved id to a [`CardInfo`]
/// so the deckbuilder can render it directly.
pub fn resolve_import(reg: &CardRegistry, text: &str) -> ImportedDeck {
    let p = parse_deck_text(text, reg);
    let proj = |entries: &[(arcana_core::types::CardId, u32)]| -> Vec<DeckEntry> {
        entries.iter()
            .filter_map(|(id, count)| card_info(reg, *id).map(|info| DeckEntry { info, count: *count }))
            .collect()
    };
    ImportedDeck {
        name: p.name,
        main: proj(&p.main),
        sideboard: proj(&p.sideboard),
        unresolved: p.unresolved,
    }
}

/// One opponent action worth showing the human ("Opponent: cast …").
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct RecentAction {
    pub player: PlayerId,
    pub description: String,
}

/// Which kind of combat decision the human is being asked for.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CombatKind {
    Attackers,
    Blockers,
    /// Order the blockers of each multi-blocked attacker (damage-assignment order).
    OrderBlockers,
    /// Distribute each attacker's combat damage across its ordered blockers.
    AssignDamage,
}

/// An attacker blocked by ≥2 creatures whose blockers must be put in
/// damage-assignment order.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct OrderingGroup {
    pub attacker: ObjectId,
    pub blockers: Vec<ObjectId>,
}

/// An attacker that must distribute its combat damage. `power` is the total to
/// assign; `targets` are its blockers already in damage-assignment order;
/// `trample` allows the sum to be less than `power` (the remainder tramples over
/// to the defending player).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct DamageGroup {
    pub attacker: ObjectId,
    pub power: u32,
    pub trample: bool,
    pub targets: Vec<ObjectId>,
}

/// One of the human's creatures that may attack, plus the entities it could
/// attack (usually just the lone opponent; a planeswalker/battle adds options).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct AttackerOption {
    pub id: ObjectId,
    pub defenders: Vec<DefendingEntity>,
}

/// One of the human's creatures that may block, plus the attackers it can be
/// declared against.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct BlockerOption {
    pub id: ObjectId,
    pub can_block: Vec<ObjectId>,
}

/// A pending combat declaration, expressed as per-creature option sets so the
/// frontend can build the declaration one click at a time (instead of choosing
/// from the engine's full enumeration of whole declarations, which is
/// combinatorial). Derived from the cached legal list via the
/// [`arcana_core::combat`] helpers, so no combat rules are duplicated client-side.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CombatPrompt {
    pub kind: CombatKind,
    /// Populated when `kind == Attackers`.
    pub attackers: Vec<AttackerOption>,
    /// Populated when `kind == Blockers`.
    pub blockers: Vec<BlockerOption>,
    /// The attacking creatures the human is defending against (for display /
    /// highlight). Populated when `kind == Blockers`.
    pub incoming: Vec<ObjectId>,
    /// Populated when `kind == OrderBlockers`.
    pub orderings: Vec<OrderingGroup>,
    /// Populated when `kind == AssignDamage`.
    pub damage: Vec<DamageGroup>,
}

/// The human's incremental combat declaration, sent back to `/combat`. The set
/// of picks is matched against the engine's enumeration; an empty set means
/// "declare no attackers / no blockers" (which is always legal).
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum CombatSubmission {
    Attackers { attackers: Vec<AttackerDeclaration> },
    Blockers { blockers: Vec<BlockerDeclaration> },
    /// Per attacker: its blockers in the chosen damage-assignment order.
    Order { orderings: Vec<(ObjectId, Vec<ObjectId>)> },
    /// Per attacker: its (blocker, amount) distribution.
    Damage { distributions: Vec<DamageAssignment> },
}

/// The JSON payload `/state`, `/action`, and `/combat` return: the human's view
/// of the game, whatever the opponent did since the human last acted, and — when
/// the pending decision is a combat declaration — the per-creature option sets
/// for the rich combat builder.
/// A heuristic evaluation of the position from the human's seat — for the
/// cockpit's eval bar. `value` is the material heuristic (~[-1, 1], terminal ±1);
/// `win_pct` is the logistic map of it to 0..100, with the scale CALIBRATED
/// against self-play outcomes (`arcana_ai::calibrate`), so it's a checked
/// estimate (referee-relative, not perfect-play truth) rather than an arbitrary
/// constant.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Eval {
    pub value: f32,
    pub win_pct: f32,
}

/// Composition of the human's own library, for the draw-odds panel. (The player
/// legitimately knows their decklist, so what's LEFT in the library is derivable;
/// the opponent's library is never exposed.) The client computes hypergeometric
/// draw odds from these counts.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct LibraryStats {
    pub total: usize,
    pub by_type: Vec<(String, usize)>,
    pub by_cmc: Vec<(u32, usize)>,
    pub by_name: Vec<(String, usize)>,
}

fn eval_for(state: &GameState, seat: PlayerId) -> Eval {
    let value = arcana_ai::search::value(state, seat);
    let win_pct = 100.0 * arcana_ai::calibrate::win_probability(value);
    Eval { value, win_pct }
}

/// `seat`'s per-permanent card power (marginal win% in this position), sorted
/// descending. Cheap (no clones/rollouts); see `card_marginal_values`.
fn card_power_for(state: &GameState, seat: PlayerId) -> Vec<CardPower> {
    arcana_ai::search::card_marginal_values(state, seat)
        .into_iter()
        .map(|(id, win_pct)| CardPower { id, win_pct })
        .collect()
}

fn library_stats(state: &GameState, reg: &CardRegistry, seat: PlayerId) -> LibraryStats {
    use std::collections::HashMap;
    let mut by_type: HashMap<&'static str, usize> = HashMap::new();
    let mut by_cmc: HashMap<u32, usize> = HashMap::new();
    let mut by_name: HashMap<String, usize> = HashMap::new();
    let mut total = 0;
    for o in state.objects.objects_in_zone(arcana_core::zones::Zone::Library(seat)) {
        total += 1;
        let c = &o.characteristics;
        let t = &c.types;
        for (label, present) in [
            ("Creature", t.is_creature()), ("Land", t.is_land()),
            ("Instant", t.is_instant()), ("Sorcery", t.is_sorcery()),
            ("Artifact", t.is_artifact()), ("Enchantment", t.is_enchantment()),
            ("Planeswalker", t.is_planeswalker()), ("Battle", t.is_battle()),
        ] {
            if present { *by_type.entry(label).or_insert(0) += 1; }
        }
        *by_cmc.entry(c.mana_value().min(20)).or_insert(0) += 1;
        let name = reg.interner().resolve(c.name).unwrap_or_default().to_string();
        *by_name.entry(name).or_insert(0) += 1;
    }
    let mut by_type: Vec<(String, usize)> = by_type.into_iter().map(|(k, v)| (k.to_string(), v)).collect();
    by_type.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    let mut by_cmc: Vec<(u32, usize)> = by_cmc.into_iter().collect();
    by_cmc.sort_by_key(|(c, _)| *c);
    let mut by_name: Vec<(String, usize)> = by_name.into_iter().collect();
    by_name.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    LibraryStats { total, by_type, by_cmc, by_name }
}

/// One ranked legal action for the cockpit's "suggested lines" panel: the human
/// action `index` (the same index `/action` consumes), its readable `label`, the
/// mean value-MC rollout `value` (~[-1, 1] from the human's seat) and its
/// calibrated `win_pct`. Sorted by value descending; the top row is the bot's
/// own pick (shared scoring kernel — see `arcana_ai::search::rank_actions`).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Suggestion {
    pub index: usize,
    pub label: String,
    pub value: f32,
    pub win_pct: f32,
}

/// Fixed RNG seed for [`GameCore::suggest`], so re-querying the SAME position
/// returns the SAME ranking (a stable analyst panel); different positions differ
/// naturally because the rolled-out state differs.
const SUGGEST_SEED: u64 = 0x5066_E57E_D11E_5;

/// One permanent's in-game "card power" for the cockpit panel: the object `id`
/// (the frontend resolves its name/art from the view) and `win_pct`, the
/// percentage points of win probability it is worth in the current position
/// (`arcana_ai::search::card_marginal_values`). Sorted descending.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CardPower {
    pub id: ObjectId,
    pub win_pct: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct StateResponse {
    pub view: ViewState,
    pub recent: Vec<RecentAction>,
    /// Heuristic position eval from the human's seat (cockpit eval bar).
    pub eval: Eval,
    /// The human's library composition (cockpit draw-odds panel).
    pub library: LibraryStats,
    /// `Some` only when the human faces a combat decision.
    pub combat: Option<CombatPrompt>,
    /// `Some(n)` when the human must put `n` cards on the bottom of their
    /// library after a London mulligan (they choose which `n` to bottom; the
    /// rest is their opening hand).
    pub bottom: Option<usize>,
    /// Per-permanent in-game card power for the human's board (cockpit panel),
    /// sorted by win% contribution descending.
    pub card_power: Vec<CardPower>,
}

/// The number of cards a London-mulligan bottoming asks for, or `None` if no
/// bottoming is pending (derived from the canonical [`Action::BottomCards`]).
fn bottom_prompt(legal: &[Action]) -> Option<usize> {
    legal.iter().find_map(|a| match a {
        Action::BottomCards(ids) => Some(ids.len()),
        _ => None,
    })
}

/// Build a [`CombatPrompt`] from the pending decision, or `None` if it isn't a
/// combat decision. The four combat decisions are mutually exclusive (the
/// helpers each return empty unless that decision is pending), so the first
/// non-empty one wins. `state` is needed for attacker power / trample on the
/// damage-assignment prompt.
fn combat_prompt(state: &GameState, legal: &[Action]) -> Option<CombatPrompt> {
    let empty = || CombatPrompt {
        kind: CombatKind::Attackers,
        attackers: Vec::new(),
        blockers: Vec::new(),
        incoming: Vec::new(),
        orderings: Vec::new(),
        damage: Vec::new(),
    };

    let atk = attacker_options(legal);
    if !atk.is_empty() {
        return Some(CombatPrompt {
            attackers: atk.into_iter()
                .map(|(id, defenders)| AttackerOption { id, defenders }).collect(),
            ..empty()
        });
    }
    let blk = blocker_options(legal);
    if !blk.is_empty() {
        let mut incoming: Vec<ObjectId> =
            blk.iter().flat_map(|(_, atkrs)| atkrs.iter().copied()).collect();
        incoming.sort_unstable();
        incoming.dedup();
        return Some(CombatPrompt {
            kind: CombatKind::Blockers,
            blockers: blk.into_iter()
                .map(|(id, can_block)| BlockerOption { id, can_block }).collect(),
            incoming,
            ..empty()
        });
    }
    let ord = ordering_targets(legal);
    if !ord.is_empty() {
        return Some(CombatPrompt {
            kind: CombatKind::OrderBlockers,
            orderings: ord.into_iter()
                .map(|(attacker, blockers)| OrderingGroup { attacker, blockers }).collect(),
            ..empty()
        });
    }
    let dmg = damage_targets(legal);
    if !dmg.is_empty() {
        return Some(CombatPrompt {
            kind: CombatKind::AssignDamage,
            damage: dmg.into_iter().map(|(attacker, targets)| DamageGroup {
                attacker,
                power: state.computed_power(attacker).unwrap_or(0).max(0) as u32,
                trample: state.has_keyword(attacker, &KeywordAbility::Trample),
                targets,
            }).collect(),
            ..empty()
        });
    }
    None
}

/// Why an `apply_index` call could not be honored. Surfaced to the client as a
/// 400, never as a panic.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ApplyError {
    /// The game is over (or no decision has been surfaced yet) — nothing to apply.
    NoPendingDecision,
    /// The index was outside the current legal-action list.
    OutOfRange { index: usize, len: usize },
    /// A submitted combat declaration didn't match any legal declaration (an
    /// illegal set, or one beyond the engine's enumeration cap). The frontend
    /// should re-prompt with the current options.
    IllegalCombat,
    /// Auto-tap was asked to play a card that can't be cast/played this turn even
    /// tapping out (it shouldn't have been clickable).
    NotPlayable { id: ObjectId },
    /// A London-mulligan bottom submission was malformed (wrong count, or an id
    /// not in the human's hand). `owed` is how many must be bottomed.
    IllegalBottom { owed: usize },
    /// The requesting seat tried to act when the pending decision belongs to a
    /// different seat (a networked client submitting out of turn). `awaiting` is
    /// the seat whose decision is actually pending, if any.
    NotYourTurn { seat: PlayerId, awaiting: Option<PlayerId> },
}

impl fmt::Display for ApplyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApplyError::NoPendingDecision => {
                write!(f, "no decision is pending (the game may be over) — start a new game")
            }
            ApplyError::OutOfRange { index, len } => {
                write!(f, "action index {index} out of range (0..{len})")
            }
            ApplyError::IllegalCombat => {
                write!(f, "that combat declaration is not legal — pick again")
            }
            ApplyError::NotPlayable { id } => {
                write!(f, "card {id} can't be played this turn")
            }
            ApplyError::IllegalBottom { owed } => {
                write!(f, "choose exactly {owed} card(s) from your hand to bottom")
            }
            ApplyError::NotYourTurn { seat, awaiting } => match awaiting {
                Some(a) => write!(f, "it's not your turn (you are seat {seat}; seat {a} is to act)"),
                None => write!(f, "no decision is pending for seat {seat}"),
            },
        }
    }
}

impl std::error::Error for ApplyError {}

/// Format a [`GameResult`] for the `game_over` banner. Mirrors the strings
/// [`view_state`] produces so the two paths agree.
pub fn format_result(r: &GameResult) -> String {
    match r {
        GameResult::Win(p) => format!("Win(P{p})"),
        GameResult::Draw => "Draw".to_string(),
        GameResult::Eliminated(p) => format!("Eliminated(P{p})"),
    }
}

// =============================================================================
// Match configuration — the "World Stage" spine
//
// Models a match as N identity-bearing SEATS so the same setup screen scales
// from a duel to a free-for-all and to networked play. The engine runs a
// two-player duel today (N-player gameplay is gated on separate engine work),
// so `GameCore::from_match_config` validates a two-seat match — but every type
// here is N-shaped, so widening it is data, not a redesign.
// =============================================================================

/// Who occupies a seat. Extensible by design: the local human names themselves
/// now; avatar / title / rating come later.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub name: String,
}
impl Default for PlayerProfile {
    fn default() -> Self {
        Self { name: "You".to_string() }
    }
}

/// A deck's *presentation* identity, shown on the Stage. Every field is DERIVED
/// from the decklist by default (colors from the mana pips, a signature card
/// for the portrait, an archetype from the curve) and OVERRIDABLE by the
/// player — the server treats a hand-edited identity exactly like a derived
/// one, so manual naming / portrait-picking / retagging "just work".
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeckIdentity {
    /// Faction / deck name (defaults to the saved deck's name).
    pub name: String,
    /// Color heraldry (WUBRG bits; derived from mana costs, overridable).
    pub colors: ColorSet,
    /// Signature card whose art is the Stage portrait (derived, overridable).
    pub portrait: Option<CardId>,
    /// "aggro" | "midrange" | "control" — derived from the curve, overridable.
    pub archetype: String,
}
impl Default for DeckIdentity {
    fn default() -> Self {
        Self {
            name: "Unnamed Deck".to_string(),
            colors: ColorSet::default(),
            portrait: None,
            archetype: String::new(),
        }
    }
}

/// Derive the DEFAULT presentation identity for a decklist — the values the
/// player then overrides on the Stage. Operates on the full multiset (repeats
/// matter for the curve); unknown ids are skipped.
///
/// * `colors`  — union of every card's colors (WUBRG heraldry).
/// * `portrait`— the signature card: highest mana value among non-lands
///   (tie-broken by name for determinism).
/// * `archetype` — "aggro" / "midrange" / "control" from the non-land curve.
/// * `name`    — the supplied saved name, else a faction-style default built
///   from the color combo + archetype ("Boros Aggro", "Esper Control").
pub fn derive_deck_identity(
    reg: &CardRegistry, deck: &[CardId], name: Option<String>,
) -> DeckIdentity {
    use arcana_core::catalog::card_info;
    let infos: Vec<_> = deck.iter().filter_map(|&id| card_info(reg, id)).collect();

    let mut colors = ColorSet::default();
    for ci in &infos {
        for &c in &ci.colors {
            colors = colors | color_from_char(c);
        }
    }

    // Signature card: the biggest non-land bomb (deterministic tie-break).
    let portrait = infos.iter()
        .filter(|ci| !ci.is_land)
        .max_by(|a, b| a.mana_value.cmp(&b.mana_value).then_with(|| b.name.cmp(&a.name)))
        .map(|ci| ci.id);

    let nonland: Vec<&arcana_core::catalog::CardInfo> =
        infos.iter().filter(|ci| !ci.is_land).collect();
    let archetype = derive_archetype(&nonland);

    let name = name.filter(|n| !n.trim().is_empty()).unwrap_or_else(|| {
        format!("{} {}", color_combo_name(colors), archetype_title(&archetype))
    });

    DeckIdentity { name, colors, portrait, archetype }
}

/// One WUBRG letter → its [`ColorSet`] bit (anything else → empty).
fn color_from_char(c: char) -> ColorSet {
    match c.to_ascii_uppercase() {
        'W' => ColorSet::white(),
        'U' => ColorSet::blue(),
        'B' => ColorSet::black(),
        'R' => ColorSet::red(),
        'G' => ColorSet::green(),
        _ => ColorSet::default(),
    }
}

/// Infer an archetype from the non-land curve. Heuristic, intentionally
/// simple — it just sets a sensible DEFAULT the player can retag:
/// * aggro   — low curve and creature-dense,
/// * control — top-heavy, or light on creatures and spell-dense,
/// * midrange— everything else.
fn derive_archetype(nonland: &[&arcana_core::catalog::CardInfo]) -> String {
    let n = nonland.len();
    if n == 0 {
        return "midrange".to_string();
    }
    let n_f = n as f64;
    let avg_mv = nonland.iter().map(|c| c.mana_value as f64).sum::<f64>() / n_f;
    let creature_ratio =
        nonland.iter().filter(|c| c.is_creature).count() as f64 / n_f;
    let spell_ratio =
        nonland.iter().filter(|c| c.is_instant || c.is_sorcery).count() as f64 / n_f;

    if avg_mv <= 2.5 && creature_ratio >= 0.5 {
        "aggro".to_string()
    } else if avg_mv >= 3.3 || (creature_ratio < 0.35 && spell_ratio >= 0.30) {
        "control".to_string()
    } else {
        "midrange".to_string()
    }
}

/// Title-case an archetype tag for the default deck name.
fn archetype_title(a: &str) -> &'static str {
    match a {
        "aggro" => "Aggro",
        "control" => "Control",
        _ => "Midrange",
    }
}

/// Faction-style name for a color combo — mono / guild / shard / wedge /
/// four-color (Nephilim) / five-color — the Civ-faction flavor on the Stage.
fn color_combo_name(colors: ColorSet) -> String {
    let order = [
        ('W', ColorSet::white()), ('U', ColorSet::blue()), ('B', ColorSet::black()),
        ('R', ColorSet::red()), ('G', ColorSet::green()),
    ];
    let present: String = order.iter()
        .filter(|(_, cs)| colors.0 & cs.0 != 0)
        .map(|(ch, _)| *ch)
        .collect();
    let name = match present.as_str() {
        "" => "Colorless",
        "W" => "Mono-White", "U" => "Mono-Blue", "B" => "Mono-Black",
        "R" => "Mono-Red", "G" => "Mono-Green",
        "WU" => "Azorius", "WB" => "Orzhov", "WR" => "Boros", "WG" => "Selesnya",
        "UB" => "Dimir", "UR" => "Izzet", "UG" => "Simic",
        "BR" => "Rakdos", "BG" => "Golgari", "RG" => "Gruul",
        "WUB" => "Esper", "WUR" => "Jeskai", "WUG" => "Bant",
        "WBR" => "Mardu", "WBG" => "Abzan", "WRG" => "Naya",
        "UBR" => "Grixis", "UBG" => "Sultai", "URG" => "Temur", "BRG" => "Jund",
        "WUBR" => "Yore", "WUBG" => "Witch", "WURG" => "Ink",
        "WBRG" => "Dune", "UBRG" => "Glint",
        "WUBRG" => "Five-Color",
        _ => return present, // unreachable given the WUBRG order
    };
    name.to_string()
}

/// WUBRG letters present in a [`ColorSet`], in canonical order — for rendering
/// color heraldry on the Stage (the frontend gets letters, not a raw bitmask).
fn color_letters(colors: ColorSet) -> Vec<char> {
    [('W', ColorSet::white()), ('U', ColorSet::blue()), ('B', ColorSet::black()),
     ('R', ColorSet::red()), ('G', ColorSet::green())]
        .iter()
        .filter(|(_, cs)| colors.0 & cs.0 != 0)
        .map(|(ch, _)| *ch)
        .collect()
}

/// The portrait card's name (for art lookup), resolved from its id.
fn portrait_name(reg: &CardRegistry, portrait: Option<CardId>) -> Option<String> {
    portrait.and_then(|id| arcana_core::catalog::card_info(reg, id)).map(|ci| ci.name)
}

/// A card the player can pick as their deck's portrait (id + display name).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CardRef {
    pub id: CardId,
    pub name: String,
}

/// A [`DeckIdentity`] plus the display extras the Stage needs but the canonical
/// type omits: the portrait card's NAME (for `/art`), the color letters (for
/// heraldry), and the deck's distinct non-land cards as portrait CANDIDATES
/// (biggest first) so the player can override the signature card. The frontend
/// posts back `identity`; the rest is render-only.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DeckIdentityView {
    pub identity: DeckIdentity,
    pub portrait_name: Option<String>,
    pub colors: Vec<char>,
    pub portrait_candidates: Vec<CardRef>,
}

/// Derive a deck's identity and wrap it with the Stage's display extras.
pub fn deck_identity_view(
    reg: &CardRegistry, deck: &[CardId], name: Option<String>,
) -> DeckIdentityView {
    let identity = derive_deck_identity(reg, deck, name);

    // Distinct non-land cards as portrait candidates, biggest mana value first
    // (then by name) — the player's signature-card picker.
    let mut seen = std::collections::HashSet::new();
    let mut cands: Vec<arcana_core::catalog::CardInfo> = Vec::new();
    for &id in deck {
        if seen.insert(id) {
            if let Some(ci) = arcana_core::catalog::card_info(reg, id) {
                if !ci.is_land {
                    cands.push(ci);
                }
            }
        }
    }
    cands.sort_by(|a, b| b.mana_value.cmp(&a.mana_value).then_with(|| a.name.cmp(&b.name)));
    let portrait_candidates = cands.into_iter()
        .map(|ci| CardRef { id: ci.id, name: ci.name })
        .collect();

    DeckIdentityView {
        portrait_name: portrait_name(reg, identity.portrait),
        colors: color_letters(identity.colors),
        portrait_candidates,
        identity,
    }
}

/// A preset AI rival shown on the World Stage — the Civ-leader gallery. A named
/// opponent with a deck, an agenda (flavor), a difficulty, and a derived deck
/// identity. Selecting one fills the opponent seat of a [`MatchConfig`] (as a
/// [`SeatSpec::Bot`]); the "custom deck" slot produces the same Bot seat from a
/// saved list, so personalities and custom decks share one code path. The
/// `portrait_name` / `colors` are render-only display extras (see
/// [`DeckIdentityView`]).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Personality {
    /// Stable key for the frontend (e.g. "pyromancer").
    pub id: String,
    pub profile: PlayerProfile,
    pub agenda: String,
    pub difficulty: Difficulty,
    pub deck: Vec<CardId>,
    pub identity: DeckIdentity,
    pub portrait_name: Option<String>,
    pub colors: Vec<char>,
}

/// The built-in roster of AI rivals. Each is a mono-color creature deck of a
/// tuned curve with a flavorful name + agenda + difficulty; the deck identity
/// (colors / portrait / archetype) is DERIVED from the real deck, so the Stage
/// portrays each rival honestly even though the flavor is authored.
pub fn personalities(reg: &CardRegistry) -> Vec<Personality> {
    // (id, name, agenda, color, difficulty, curve cap)
    const ROSTER: [(&str, &str, &str, char, Difficulty, u32); 5] = [
        ("pyromancer", "The Pyromancer", "Burn fast, burn bright.", 'R', Difficulty::Normal, 4),
        ("wildspeaker", "The Wildspeaker", "The wilds answer my call.", 'G', Difficulty::Easy, 6),
        ("cleric", "The Cleric", "Stand behind the wall of faith.", 'W', Difficulty::Easy, 4),
        ("necromancer", "The Necromancer", "Death is only the beginning.", 'B', Difficulty::Normal, 5),
        ("tempest", "The Tempest", "The tide turns at my command.", 'U', Difficulty::Hard, 6),
    ];
    ROSTER.iter().map(|&(id, name, agenda, color, difficulty, cmc_max)| {
        let deck = arcana_ai::deckeval::mono_color_creature_deck(
            reg, color, /*distinct=*/ 12, /*n_spells=*/ 22, /*n_lands=*/ 18, cmc_max,
        ).cards;
        let identity = derive_deck_identity(reg, &deck, None);
        Personality {
            id: id.to_string(),
            profile: PlayerProfile { name: name.to_string() },
            agenda: agenda.to_string(),
            difficulty,
            portrait_name: portrait_name(reg, identity.portrait),
            colors: color_letters(identity.colors),
            deck,
            identity,
        }
    }).collect()
}

/// Bot strength dial → Monte-Carlo search budget.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub enum Difficulty {
    Easy,
    #[default]
    Normal,
    Hard,
}

/// One seat of a match. A `#[serde(tag = "kind")]` enum so the Stage frontend
/// sends `{ "kind": "Bot", ... }`.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum SeatSpec {
    /// The local human (this client).
    Local {
        #[serde(default)]
        profile: PlayerProfile,
        deck: Vec<CardId>,
        #[serde(default)]
        identity: DeckIdentity,
    },
    /// An AI rival — a named personality or a custom deck — at a difficulty.
    Bot {
        #[serde(default)]
        profile: PlayerProfile,
        #[serde(default)]
        agenda: String,
        deck: Vec<CardId>,
        #[serde(default)]
        identity: DeckIdentity,
        #[serde(default)]
        difficulty: Difficulty,
    },
    /// A networked human. The deck arrives over the wire; a stub until the
    /// multiplayer phase wires real transport.
    Network {
        #[serde(default)]
        profile: PlayerProfile,
        #[serde(default)]
        identity: DeckIdentity,
    },
}

/// Who takes the first turn.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum FirstPlayer {
    Seat { index: usize },
    Random,
}
impl Default for FirstPlayer {
    fn default() -> Self {
        FirstPlayer::Random
    }
}

/// A full match setup posted from the World Stage.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct MatchConfig {
    pub seats: Vec<SeatSpec>,
    #[serde(default)]
    pub first_player: FirstPlayer,
    #[serde(default)]
    pub seed: u64,
}

/// One in-progress game: a human (P0) vs a snappy Monte-Carlo bot (P1) on
/// mirrored sample decks. Holds the legal actions from the most recent human
/// decision so `/action` can resolve an index the browser sent back.
pub struct GameCore {
    reg: &'static CardRegistry,
    session: Session<'static>,
    /// Legal actions surfaced at the latest human decision, indexed by the
    /// `index` the frontend echoes back. Empty once the game is over.
    legal: Vec<Action>,
    /// The seat whose decision `legal` belongs to (the player to act), or `None`
    /// when no human decision is pending. With two networked humans sharing one
    /// `GameCore`, a client may only act when it owns the pending decision; this
    /// is what `*_for(seat, …)` validates against (`ApplyError::NotYourTurn`).
    awaiting: Option<PlayerId>,
}

impl GameCore {
    /// Build the snappy bot policy recommended for interactive play (Normal).
    fn make_bot(seed: u64) -> Seat {
        Self::make_bot_with_difficulty(seed, Difficulty::Normal)
    }

    /// Bot policy at a chosen [`Difficulty`] — the dial scales the Monte-Carlo
    /// search budget (rollouts / depth cap / candidate breadth). Even "Hard"
    /// stays light enough to answer an HTTP request promptly.
    fn make_bot_with_difficulty(seed: u64, difficulty: Difficulty) -> Seat {
        let (rollouts, cap, candidates) = match difficulty {
            Difficulty::Easy => (2, 15, 6),
            Difficulty::Normal => (6, 25, 10),
            Difficulty::Hard => (20, 40, 16),
        };
        let policy = ValueMcPolicy::with_budget(
            Box::new(MaterialValue), seed ^ 0xA5EED, rollouts, cap, candidates);
        Seat::Bot(Box::new(policy))
    }

    /// Build a game from a [`MatchConfig`] (the World Stage's output). The
    /// config models N seats, but the engine runs a two-player duel today, so
    /// this requires exactly two seats: seat 0 the local human, seat 1 an AI
    /// opponent (a rival personality or a custom deck). Returns an error string
    /// the HTTP layer can surface (a `400`) rather than panicking on bad input.
    ///
    /// `first_player`: the engine starts seat 0 today; `Random` perturbs the
    /// seed so the shuffle differs. Seat-controlled first-player is a follow-up
    /// (it needs an engine starting-player parameter).
    pub fn from_match_config(
        reg: &'static CardRegistry, cfg: &MatchConfig,
    ) -> Result<Self, String> {
        if cfg.seats.len() != 2 {
            return Err(format!(
                "a duel needs exactly two seats; got {}", cfg.seats.len()));
        }
        let human_deck = match &cfg.seats[0] {
            SeatSpec::Local { deck, .. } => deck.clone(),
            _ => return Err("seat 0 must be the local human".to_string()),
        };
        let (opp_deck, difficulty) = match &cfg.seats[1] {
            SeatSpec::Bot { deck, difficulty, .. } => (deck.clone(), *difficulty),
            SeatSpec::Network { .. } =>
                return Err("network opponents aren't supported yet".to_string()),
            SeatSpec::Local { .. } =>
                return Err("seat 1 must be an opponent, not a second local seat".to_string()),
        };
        if human_deck.is_empty() || opp_deck.is_empty() {
            return Err("both decks must be non-empty".to_string());
        }
        let seed = match cfg.first_player {
            FirstPlayer::Random => cfg.seed ^ 0xF1257, // perturb the shuffle
            FirstPlayer::Seat { .. } => cfg.seed,
        };
        let seats = vec![Seat::Human, Self::make_bot_with_difficulty(seed, difficulty)];
        let session = Session::new(vec![human_deck, opp_deck], reg, seats, seed);
        Ok(Self { reg, session, legal: Vec::new(), awaiting: None })
    }

    /// Start a fresh game. `seed` controls the shuffle/RNG (the deck list itself
    /// is fixed by [`DECK_SEED`] so both seats play the same 40 cards).
    pub fn new(reg: &'static CardRegistry, seed: u64) -> Self {
        Self::new_with_deck(reg, seed, arcana_cards::sample_deck(reg, DECK_SEED))
    }

    /// Start a fresh game where both seats play `deck` (a mirror match). Human in
    /// seat 0, snappy bot in seat 1.
    pub fn new_with_deck(reg: &'static CardRegistry, seed: u64, deck: Vec<CardId>) -> Self {
        Self::new_with_decks(reg, seed, deck.clone(), deck)
    }

    /// Start a fresh game where the human (seat 0) plays `human` and the bot
    /// (seat 1) plays `opponent` — the deckbuilder's "play my deck vs X".
    pub fn new_with_decks(
        reg: &'static CardRegistry, seed: u64, human: Vec<CardId>, opponent: Vec<CardId>,
    ) -> Self {
        let seats = vec![Seat::Human, Self::make_bot(seed)];
        let session = Session::new(vec![human, opponent], reg, seats, seed);
        Self { reg, session, legal: Vec::new(), awaiting: None }
    }

    /// Read-only access to the registry (for callers that build a replacement
    /// `GameCore` on `/new`).
    pub fn registry(&self) -> &'static CardRegistry {
        self.reg
    }

    /// Set the human auto-pass level (none / middle-ground / full).
    pub fn set_auto_pass(&mut self, level: arcana_ai::session::AutoPass) {
        self.session.set_auto_pass(level);
    }

    /// Backward-compatible snapshot from the local human's seat ([`HUMAN`]) —
    /// the solo vs-AI path. See [`snapshot_for`](Self::snapshot_for).
    pub fn snapshot(&mut self) -> StateResponse {
        self.snapshot_for(HUMAN)
    }

    /// Drive the session through all bot + trivial decisions, then project the
    /// game from `seat`'s perspective into a [`StateResponse`].
    ///
    /// Idempotent while a human decision is pending: `advance` re-surfaces the
    /// same decision without mutating, so polling `/state` is safe — and two
    /// networked clients can each poll their own `seat` against the one game.
    ///
    /// When the pending decision belongs to `seat`, the response carries that
    /// seat's legal actions / combat / mulligan prompts. When it belongs to the
    /// OTHER seat, `seat` sees its own board (own hand visible, opponent's
    /// hidden) with no actions — a "waiting for opponent" view.
    pub fn snapshot_for(&mut self, seat: PlayerId) -> StateResponse {
        let view = match self.session.advance() {
            Turn::AwaitingHuman { player, view, legal, .. } => {
                self.legal = legal;
                self.awaiting = Some(player);
                if player == seat {
                    // This client is the one to act.
                    let mut vs = view_state(&view.state, self.reg, seat, &self.legal);
                    // `view.state` anonymizes hidden zones; a search lets the
                    // searching player see the real cards, so recompute the
                    // picker from the authoritative state (ids are preserved).
                    vs.choice = build_choice_view(
                        self.session.state(), self.reg, seat, &self.legal);
                    vs
                } else {
                    // The other seat is to act — project this seat's own view
                    // (its hand visible, opponent's hidden) with no actions.
                    let projected = project(self.session.state(), seat);
                    view_state(&projected.state, self.reg, seat, &[])
                }
            }
            Turn::GameOver(result) => {
                self.legal = Vec::new();
                self.awaiting = None;
                let projected = project(self.session.state(), seat);
                let mut vs = view_state(&projected.state, self.reg, seat, &[]);
                if vs.game_over.is_none() {
                    vs.game_over = Some(format_result(&result));
                }
                vs
            }
        };
        let recent = self
            .session
            .recent_actions()
            .iter()
            .map(|(p, d)| RecentAction { player: *p, description: d.clone() })
            .collect();
        // Combat / mulligan prompts only belong to the seat whose decision is
        // pending; a waiting client must not be shown the actor's prompt.
        let is_my_turn = self.awaiting == Some(seat);
        let combat = if is_my_turn { combat_prompt(self.session.state(), &self.legal) } else { None };
        let bottom = if is_my_turn { bottom_prompt(&self.legal) } else { None };
        let eval = eval_for(self.session.state(), seat);
        let library = library_stats(self.session.state(), self.reg, seat);
        let card_power = card_power_for(self.session.state(), seat);
        StateResponse { view, recent, eval, library, combat, bottom, card_power }
    }

    /// Rank the human's current legal actions by value-MC lookahead for the
    /// cockpit's "suggested lines" panel (see `arcana_ai::search::rank_actions`).
    /// Returns `[]` unless the human faces a real (≥2-way) choice. `deep` trades
    /// a bigger rollout budget for a closer look (the panel's "deepen" control).
    ///
    /// Reads the cached `legal` from the last [`snapshot`](Self::snapshot) (same
    /// invariant `/action` relies on) and the true [`session`](Session) state, so
    /// rollouts are perfect-information from the human's seat — a documented
    /// simplification (the rollout engine sees the opponent's hidden cards).
    pub fn suggest(&self, deep: bool) -> Vec<Suggestion> {
        self.suggest_for(HUMAN, deep)
    }

    /// As [`suggest`](Self::suggest) but ranked from `seat`'s perspective. Only
    /// meaningful when the cached `legal` belongs to `seat` (its decision is
    /// pending); returns `[]` otherwise.
    pub fn suggest_for(&self, seat: PlayerId, deep: bool) -> Vec<Suggestion> {
        if self.legal.len() <= 1 || self.awaiting != Some(seat) {
            return Vec::new();
        }
        let state = self.session.state();
        // Snappy auto budget vs a heavier "deepen" budget.
        let (rollouts, cap, candidates) = if deep { (20, 40, 16) } else { (8, 30, 12) };
        arcana_ai::search::rank_actions(
            state, self.reg, seat, &self.legal, &MaterialValue,
            rollouts, cap, candidates, SUGGEST_SEED,
        )
        .into_iter()
        .map(|s| Suggestion {
            index: s.index,
            label: arcana_core::render::render_action(&self.legal[s.index], state, self.reg),
            value: s.value,
            win_pct: s.win_pct,
        })
        .collect()
    }

    /// Apply the human's chosen action by its `index` into the legal list from
    /// the most recent [`snapshot`](Self::snapshot), then advance and snapshot
    /// again. Returns the new state, or an [`ApplyError`] for a stale/out-of-range
    /// index or a finished game — never panics on bad input.
    pub fn apply_index(&mut self, index: usize) -> Result<StateResponse, ApplyError> {
        self.apply_index_for(HUMAN, index)
    }

    /// As [`apply_index`](Self::apply_index) but for a specific `seat`: rejects
    /// the call with [`ApplyError::NotYourTurn`] unless the pending decision
    /// belongs to `seat` (so a networked client can't act out of turn).
    pub fn apply_index_for(&mut self, seat: PlayerId, index: usize) -> Result<StateResponse, ApplyError> {
        self.ensure_turn(seat)?;
        let action = self
            .legal
            .get(index)
            .cloned()
            .ok_or(ApplyError::OutOfRange { index, len: self.legal.len() })?;
        self.session.apply(action);
        Ok(self.snapshot_for(seat))
    }

    /// Guard a seat-scoped mutation: the game must have a pending decision and it
    /// must belong to `seat`.
    fn ensure_turn(&self, seat: PlayerId) -> Result<(), ApplyError> {
        if self.legal.is_empty() {
            return Err(ApplyError::NoPendingDecision);
        }
        if self.awaiting != Some(seat) {
            return Err(ApplyError::NotYourTurn { seat, awaiting: self.awaiting });
        }
        Ok(())
    }

    /// Apply an incremental combat declaration built by the frontend. The picked
    /// set is matched against the cached legal enumeration via the
    /// [`arcana_core::combat`] matchers (so all combat rules — lethal ordering,
    /// trample, the 1024-declaration cap — stay engine-side); an empty set is the
    /// always-legal "no attacks / no blocks". Returns [`ApplyError::IllegalCombat`]
    /// if the set isn't a legal declaration, so the frontend can re-prompt.
    pub fn apply_combat(&mut self, sub: CombatSubmission) -> Result<StateResponse, ApplyError> {
        self.apply_combat_for(HUMAN, sub)
    }

    /// As [`apply_combat`](Self::apply_combat) but for a specific `seat`
    /// (validated against the pending decision).
    pub fn apply_combat_for(&mut self, seat: PlayerId, sub: CombatSubmission) -> Result<StateResponse, ApplyError> {
        self.ensure_turn(seat)?;
        let action = match sub {
            CombatSubmission::Attackers { attackers } => match_attack(&self.legal, &attackers),
            CombatSubmission::Blockers { blockers } =>
                legal_block_declaration(self.session.state(), seat, &blockers),
            CombatSubmission::Order { orderings } => match_ordering(&self.legal, &orderings),
            CombatSubmission::Damage { distributions } => match_damage(&self.legal, &distributions),
        };
        let action = action.ok_or(ApplyError::IllegalCombat)?;
        self.session.apply(action);
        Ok(self.snapshot_for(seat))
    }

    /// MTGA-style "click a card to play it": auto-tap the mana to make hand card
    /// `target` castable, then cast/play it if there's a single way to — else
    /// leave the mana floated and surface the now-available cast variants (e.g.
    /// to choose targets). Returns [`ApplyError::NotPlayable`] if it can't be
    /// played this turn. See [`arcana_core::legal_actions::auto_tap_sequence`].
    pub fn auto_tap_and_cast(&mut self, target: ObjectId) -> Result<StateResponse, ApplyError> {
        self.auto_tap_and_cast_for(HUMAN, target)
    }

    /// As [`auto_tap_and_cast`](Self::auto_tap_and_cast) but for a specific `seat`.
    pub fn auto_tap_and_cast_for(&mut self, seat: PlayerId, target: ObjectId) -> Result<StateResponse, ApplyError> {
        self.ensure_turn(seat)?;
        let seq = arcana_core::legal_actions::auto_tap_sequence(
            self.session.state(), self.reg, seat, target)
            .ok_or(ApplyError::NotPlayable { id: target })?;
        for action in seq {
            self.session.apply(action);
        }
        Ok(self.snapshot_for(seat))
    }

    /// Auto-tap mana for a permanent's (non-mana) activated ability, then activate
    /// it — or, if the ability needs a target/mode choice, just float the mana so
    /// the per-choice activations become legal (the frontend then surfaces them).
    /// See [`arcana_core::legal_actions::auto_tap_activate_sequence`].
    pub fn auto_tap_and_activate(&mut self, source: ObjectId) -> Result<StateResponse, ApplyError> {
        self.auto_tap_and_activate_for(HUMAN, source)
    }

    /// As [`auto_tap_and_activate`](Self::auto_tap_and_activate) but for a specific `seat`.
    pub fn auto_tap_and_activate_for(&mut self, seat: PlayerId, source: ObjectId) -> Result<StateResponse, ApplyError> {
        self.ensure_turn(seat)?;
        let seq = arcana_core::legal_actions::auto_tap_activate_sequence(
            self.session.state(), self.reg, seat, source)
            .ok_or(ApplyError::NotPlayable { id: source })?;
        for action in seq {
            self.session.apply(action);
        }
        Ok(self.snapshot_for(seat))
    }

    /// Apply a London-mulligan bottoming: put the chosen `ids` on the bottom of
    /// the human's library (the rest is their opening hand). Validated against the
    /// owed count and the human's hand — `apply_bottom_cards` panics on a bad
    /// submission, so a malformed request is rejected as [`ApplyError::IllegalBottom`]
    /// rather than reaching the engine.
    pub fn bottom_cards(&mut self, ids: Vec<ObjectId>) -> Result<StateResponse, ApplyError> {
        self.bottom_cards_for(HUMAN, ids)
    }

    /// As [`bottom_cards`](Self::bottom_cards) but for a specific `seat`.
    pub fn bottom_cards_for(&mut self, seat: PlayerId, ids: Vec<ObjectId>) -> Result<StateResponse, ApplyError> {
        self.ensure_turn(seat)?;
        let owed = bottom_prompt(&self.legal).ok_or(ApplyError::NoPendingDecision)?;
        let hand: std::collections::HashSet<ObjectId> = self
            .session
            .state()
            .objects
            .iter()
            .filter(|o| o.zone == arcana_core::zones::Zone::Hand(seat))
            .map(|o| o.id)
            .collect();
        let mut seen = std::collections::HashSet::new();
        let valid = ids.len() == owed
            && ids.iter().all(|id| hand.contains(id) && seen.insert(*id));
        if !valid {
            return Err(ApplyError::IllegalBottom { owed });
        }
        self.session.apply(Action::BottomCards(ids));
        Ok(self.snapshot_for(seat))
    }

    /// The seat whose decision is currently pending (the player to act), or
    /// `None` if the game is over / no human decision is up. Networked routing
    /// uses this to tell a polling client whether it's their turn.
    pub fn awaiting_seat(&self) -> Option<PlayerId> {
        self.awaiting
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // A leaked registry mirrors the server's `'static` app-state strategy.
    fn leaked_catalog() -> &'static CardRegistry {
        Box::leak(Box::new(arcana_cards::build_catalog()))
    }

    /// Covers all three required guarantees of the server logic:
    ///  (1) `/state` returns a `ViewState` (two players, human perspective),
    ///  (2) applying a legal index advances the game across many decisions,
    ///  (3) an out-of-range index is rejected gracefully (Err, not panic).
    #[test]
    fn state_action_flow_and_bad_index() {
        let reg = leaked_catalog();
        let mut core = GameCore::new(reg, 42);

        // (1) /state yields a coherent ViewState from the human's perspective.
        let s = core.snapshot();
        assert!(s.view.game_over.is_none(), "a fresh game is not over");
        assert_eq!(s.view.players.len(), 2, "two players");
        assert_eq!(s.view.perspective, HUMAN, "rendered from the human's seat");
        assert_eq!(s.view.players[HUMAN as usize].life, 20, "starting life");
        assert!(!s.view.legal.is_empty(), "the human faces a real decision");
        assert!(s.view.legal.iter().all(|a| !a.label.is_empty()), "labels are populated");
        // The own hand carries card names (for art lookup); opponent's is hidden.
        assert_eq!(s.view.players[0].hand.len(), s.view.players[0].hand_count);
        assert!(s.view.players[1].hand.is_empty(), "opponent hand stays hidden");

        // (3) Out-of-range indices are rejected gracefully — no panic.
        let len = s.view.legal.len();
        assert_eq!(core.apply_index(len), Err(ApplyError::OutOfRange { index: len, len }));
        assert!(matches!(core.apply_index(usize::MAX), Err(ApplyError::OutOfRange { .. })));

        // (2) Applying legal indices advances the game: drive a bounded run of
        // human decisions, each returning a coherent ViewState. Confirm play
        // actually progressed past the opening (more than one human decision and
        // the turn counter moved forward).
        let opening_turn = s.view.turn;
        let mut cur = s;
        let mut decisions = 0;
        let mut max_turn = opening_turn;
        while cur.view.game_over.is_none() && decisions < 120 {
            decisions += 1;
            // Prefer a real action over Pass/Concede/Mulligan-away so we develop.
            let idx = cur
                .view
                .legal
                .iter()
                .position(|a| {
                    let l = a.label.as_str();
                    l != "Pass" && l != "Concede" && l != "Mulligan (draw a new hand)"
                })
                .unwrap_or(0);
            cur = core.apply_index(idx).expect("a legal index always applies");
            assert_eq!(cur.view.players.len(), 2, "view stays coherent across steps");
            max_turn = max_turn.max(cur.view.turn);
        }
        assert!(decisions > 1, "the human took multiple advancing decisions");
        assert!(max_turn >= opening_turn, "turn counter never regressed");
    }

    /// `apply_index` before any `snapshot` (no decision cached) is a clean Err,
    /// not a panic.
    #[test]
    fn apply_before_any_state_is_graceful() {
        let reg = leaked_catalog();
        let mut core = GameCore::new(reg, 1);
        assert_eq!(core.apply_index(0), Err(ApplyError::NoPendingDecision));
    }

    /// The combat-prompt builder and the matcher agree, and a non-combat legal
    /// list yields no prompt. Pure (no session) so it's deterministic.
    #[test]
    fn combat_prompt_and_matcher_are_consistent() {
        let st = GameState::new(2, 0); // state unused by attacker detection
        // No legal actions / non-combat decisions produce no prompt.
        assert!(combat_prompt(&st, &[]).is_none());

        let mk = || AttackerDeclaration { attacker: 5, defending: DefendingEntity::Player(1) };
        let legal = vec![
            Action::DeclareAttackers { attackers: vec![] }, // the always-legal "no attacks"
            Action::DeclareAttackers { attackers: vec![mk()] },
        ];
        let prompt = combat_prompt(&st, &legal).expect("an attacker prompt");
        assert_eq!(prompt.kind, CombatKind::Attackers);
        assert_eq!(prompt.attackers.len(), 1);
        assert_eq!(prompt.attackers[0].id, 5);
        assert_eq!(prompt.attackers[0].defenders, vec![DefendingEntity::Player(1)]);
        assert!(prompt.blockers.is_empty());

        // The matcher recovers the declaration the prompt advertised, and the
        // empty set (declare no attackers) is always available here.
        assert!(match_attack(&legal, &[mk()]).is_some());
        assert!(match_attack(&legal, &[]).is_some());
    }

    /// End-to-end combat: play a real game and, whenever a combat prompt is
    /// surfaced, push an actual legal declaration back through the
    /// builder→matcher→apply path. Confirms the server combat wiring drives a
    /// live session, not just a synthetic legal list.
    #[test]
    fn combat_prompt_and_apply_roundtrip_in_a_real_game() {
        let reg = leaked_catalog();
        let mut core = GameCore::new(reg, 11);
        let mut cur = core.snapshot();
        let mut saw_combat = false;
        let mut decisions = 0;

        while cur.view.game_over.is_none() && decisions < 600 {
            decisions += 1;
            if let Some(prompt) = cur.combat.clone() {
                saw_combat = true;
                // Replay an actual enumerated declaration through the public
                // submission path (the field is in-module-visible to the test).
                let sub = match prompt.kind {
                    CombatKind::Attackers => {
                        let attackers = core
                            .legal
                            .iter()
                            .find_map(|a| match a {
                                Action::DeclareAttackers { attackers } => Some(attackers.clone()),
                                _ => None,
                            })
                            .expect("a DeclareAttackers is enumerated for an attacker prompt");
                        CombatSubmission::Attackers { attackers }
                    }
                    CombatKind::Blockers => {
                        let blockers = core
                            .legal
                            .iter()
                            .find_map(|a| match a {
                                Action::DeclareBlockers { blockers } => Some(blockers.clone()),
                                _ => None,
                            })
                            .expect("a DeclareBlockers is enumerated for a blocker prompt");
                        CombatSubmission::Blockers { blockers }
                    }
                    CombatKind::OrderBlockers => {
                        let orderings = core
                            .legal
                            .iter()
                            .find_map(|a| match a {
                                Action::OrderBlockers { orderings } => Some(orderings.clone()),
                                _ => None,
                            })
                            .expect("an OrderBlockers is enumerated for an ordering prompt");
                        CombatSubmission::Order { orderings }
                    }
                    CombatKind::AssignDamage => {
                        let distributions = core
                            .legal
                            .iter()
                            .find_map(|a| match a {
                                Action::AssignCombatDamage { distributions } =>
                                    Some(distributions.clone()),
                                _ => None,
                            })
                            .expect("an AssignCombatDamage is enumerated for a damage prompt");
                        CombatSubmission::Damage { distributions }
                    }
                };
                cur = core.apply_combat(sub).expect("a real legal declaration applies");
                continue;
            }
            // Non-combat: greedily take a developing action so we reach combat.
            let idx = cur
                .view
                .legal
                .iter()
                .position(|a| {
                    let l = a.label.as_str();
                    l != "Pass" && l != "Concede" && l != "Mulligan (draw a new hand)"
                })
                .unwrap_or(0);
            cur = core.apply_index(idx).expect("a legal index applies");
        }

        assert!(saw_combat, "expected at least one combat declaration in a full game");
    }

    /// Auto-tap: drive to the first state with a playable hand card, click it,
    /// and confirm it leaves the hand (a land played / spell cast). Also confirms
    /// a non-playable card is rejected gracefully.
    #[test]
    fn auto_tap_plays_a_playable_card() {
        let reg = leaked_catalog();
        let mut core = GameCore::new(reg, 11);

        // At the opening (mulligan), nothing is playable → auto-tap is rejected.
        let s = core.snapshot();
        let some_card = s.view.players[0].hand[0].id;
        assert!(matches!(core.auto_tap_and_cast(some_card),
            Err(ApplyError::NotPlayable { .. })),
            "can't auto-tap during the mulligan");

        // Advance (keeping the hand, taking real actions) until a hand card is
        // flagged playable — on turn one that's a land.
        let mut cur = s;
        let mut guard = 0;
        loop {
            guard += 1;
            assert!(guard < 300, "should reach a playable card");
            assert!(cur.view.game_over.is_none(), "game ended before a playable card");
            // Turn one: the first playable card is a land. Auto-tap it and
            // confirm it entered the human's battlefield. (Battlefield is a clean
            // signal — the bot's interleaved turn never adds to the human's side,
            // whereas hand_count is confounded by the human's next-turn draw.)
            if let Some(card) = cur.view.players[0].hand.iter().find(|c| c.playable && c.is_land) {
                let id = card.id;
                let before_bf = cur.view.players[0].battlefield.len();
                let after = core.auto_tap_and_cast(id).expect("a playable land plays");
                assert!(after.view.players[0].battlefield.len() > before_bf,
                    "the auto-tapped land entered the battlefield");
                return;
            }
            let idx = cur.view.legal.iter().position(|a| {
                let l = a.label.as_str();
                l != "Pass" && l != "Concede" && l != "Mulligan (draw a new hand)"
            }).unwrap_or(0);
            cur = core.apply_index(idx).expect("legal index applies");
        }
    }

    /// London-mulligan bottoming: after one mulligan + keep the human is asked to
    /// bottom one card (their choice), and the choice is validated.
    #[test]
    fn mulligan_bottoming_surfaced_and_validated() {
        let reg = leaked_catalog();
        let mut core = GameCore::new(reg, 5);
        let s = core.snapshot();
        // Mulligan once.
        let again = s.view.legal.iter()
            .position(|a| a.label == "Mulligan (draw a new hand)").expect("can mulligan");
        let s = core.apply_index(again).expect("mulligan applies");
        // Keep the new hand.
        let keep = s.view.legal.iter()
            .position(|a| a.label == "Keep this hand").expect("can keep");
        let s = core.apply_index(keep).expect("keep applies");

        // The human must now choose ONE card to bottom — not auto-resolved.
        assert_eq!(s.bottom, Some(1), "one mulligan + keep => bottom 1, surfaced to the human");
        assert!(!s.view.players[0].hand.is_empty(), "the full kept hand is shown to choose from");

        // Malformed submissions are rejected, not panicked.
        assert!(matches!(core.bottom_cards(vec![]), Err(ApplyError::IllegalBottom { owed: 1 })));
        assert!(matches!(core.bottom_cards(vec![u32::MAX]), Err(ApplyError::IllegalBottom { .. })));

        // A valid choice (any one hand card) applies and clears the bottoming.
        let card = s.view.players[0].hand[0].id;
        let after = core.bottom_cards(vec![card]).expect("valid bottom applies");
        assert_eq!(after.bottom, None, "bottoming resolved");
    }

    /// A game started with a custom deck actually plays that deck: the human's
    /// zones account for exactly the deck's cards.
    #[test]
    fn new_with_deck_uses_the_given_deck() {
        let reg = leaked_catalog();
        let deck = arcana_cards::sample_deck(reg, 3);
        let mut core = GameCore::new_with_deck(reg, 1, deck.clone());
        let s = core.snapshot();
        let p0 = &s.view.players[0];
        assert_eq!(p0.library_count + p0.hand_count, deck.len(),
            "the human's library + hand equal the custom deck size");
        assert!(s.view.game_over.is_none());
        assert!(!s.view.legal.is_empty());
    }

    /// Each seat plays its own deck: the human's and opponent's zones each
    /// account for their distinct deck's cards.
    #[test]
    fn new_with_decks_gives_each_seat_its_deck() {
        let reg = leaked_catalog();
        let human = arcana_cards::sample_deck(reg, 3);
        let mut opponent = arcana_cards::sample_deck(reg, 9);
        opponent.truncate(opponent.len().saturating_sub(5)); // make the decks differ in size
        assert_ne!(human.len(), opponent.len(), "decks should differ for the test");

        let mut core = GameCore::new_with_decks(reg, 1, human.clone(), opponent.clone());
        let s = core.snapshot();
        let p0 = &s.view.players[0];
        let p1 = &s.view.players[1];
        assert_eq!(p0.library_count + p0.hand_count, human.len(), "seat 0 plays the human deck");
        assert_eq!(p1.library_count + p1.hand_count, opponent.len(), "seat 1 plays the opponent deck");
    }

    /// A MatchConfig builds a duel: seat 0 (local human) plays its deck, seat 1
    /// (a bot rival) plays its own. Bad configs return Err, not panic.
    #[test]
    fn from_match_config_builds_a_duel_and_rejects_bad_input() {
        let reg = leaked_catalog();
        let human = arcana_cards::sample_deck(reg, 3);
        let opp = arcana_cards::sample_deck(reg, 9);
        let local = |deck: Vec<CardId>| SeatSpec::Local {
            profile: PlayerProfile { name: "Levi".into() },
            deck, identity: DeckIdentity::default(),
        };
        let cfg = MatchConfig {
            seats: vec![
                local(human.clone()),
                SeatSpec::Bot {
                    profile: PlayerProfile { name: "The Pyromancer".into() },
                    agenda: "Burn it all.".into(),
                    deck: opp.clone(),
                    identity: DeckIdentity::default(),
                    difficulty: Difficulty::Hard,
                },
            ],
            first_player: FirstPlayer::Random,
            seed: 7,
        };
        let mut core = GameCore::from_match_config(reg, &cfg).expect("valid duel");
        let s = core.snapshot();
        assert_eq!(s.view.players[0].library_count + s.view.players[0].hand_count,
            human.len(), "seat 0 plays the human deck");
        assert!(s.view.game_over.is_none());

        // One seat → error.
        let one = MatchConfig { seats: vec![local(human.clone())], ..Default::default() };
        assert!(GameCore::from_match_config(reg, &one).is_err());
        // Network opponent → not yet supported.
        let net = MatchConfig {
            seats: vec![local(human.clone()),
                SeatSpec::Network { profile: PlayerProfile::default(),
                    identity: DeckIdentity::default() }],
            ..Default::default()
        };
        assert!(GameCore::from_match_config(reg, &net).is_err());
    }

    /// The Stage's wire contract: a MatchConfig (with the `#[serde(tag="kind")]`
    /// seats) round-trips through JSON, so the frontend can post exactly this.
    #[test]
    fn match_config_json_roundtrips() {
        let cfg = MatchConfig {
            seats: vec![
                SeatSpec::Local { profile: PlayerProfile { name: "Levi".into() },
                    deck: vec![1, 2, 3], identity: DeckIdentity::default() },
                SeatSpec::Bot { profile: PlayerProfile { name: "Rival".into() },
                    agenda: "x".into(), deck: vec![4, 5],
                    identity: DeckIdentity::default(), difficulty: Difficulty::Easy },
            ],
            first_player: FirstPlayer::Seat { index: 0 },
            seed: 42,
        };
        let json = serde_json::to_string(&cfg).expect("serialize");
        assert!(json.contains("\"kind\":\"Local\"") && json.contains("\"kind\":\"Bot\""));
        let back: MatchConfig = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back.seats.len(), 2);
        assert!(matches!(back.seats[0], SeatSpec::Local { .. }));
        assert!(matches!(back.first_player, FirstPlayer::Seat { index: 0 }));
    }

    #[test]
    fn color_combo_names_are_factions() {
        assert_eq!(color_combo_name(ColorSet::red()), "Mono-Red");
        assert_eq!(color_combo_name(ColorSet::white() | ColorSet::red()), "Boros");
        assert_eq!(color_combo_name(
            ColorSet::white() | ColorSet::blue() | ColorSet::black()), "Esper");
        assert_eq!(color_combo_name(
            ColorSet::white() | ColorSet::black() | ColorSet::red()), "Mardu");
        assert_eq!(color_combo_name(ColorSet::default()), "Colorless");
        let five = ColorSet::white() | ColorSet::blue() | ColorSet::black()
            | ColorSet::red() | ColorSet::green();
        assert_eq!(color_combo_name(five), "Five-Color");
    }

    #[test]
    fn derive_deck_identity_from_sample_deck() {
        let reg = leaked_catalog();
        let deck = arcana_cards::sample_deck(reg, 3);
        assert!(!deck.is_empty());
        let id = derive_deck_identity(reg, &deck, None);
        assert_ne!(id.colors.0, 0, "a sample deck has colored cards");
        assert!(id.portrait.is_some(), "a deck has a signature non-land card");
        assert!(!id.name.trim().is_empty(), "a default faction name is produced");
        assert!(["aggro", "midrange", "control"].contains(&id.archetype.as_str()),
            "archetype is one of the three: {}", id.archetype);
        // A supplied (saved) name overrides the derived default.
        let named = derive_deck_identity(reg, &deck, Some("My Brew".into()));
        assert_eq!(named.name, "My Brew");
    }

    #[test]
    fn personalities_roster_is_well_formed() {
        let reg = leaked_catalog();
        let roster = personalities(reg);
        assert!(roster.len() >= 5, "a few named rivals exist");
        let mut ids = std::collections::HashSet::new();
        for p in &roster {
            assert!(ids.insert(p.id.clone()), "rival ids are unique: {}", p.id);
            assert!(!p.profile.name.trim().is_empty(), "{} has a name", p.id);
            assert!(!p.agenda.trim().is_empty(), "{} has an agenda", p.id);
            assert!(!p.deck.is_empty(), "{} has a deck", p.id);
            assert_ne!(p.identity.colors.0, 0, "{} has a colored identity", p.id);
            assert!(p.identity.portrait.is_some(), "{} has a signature card", p.id);
        }
        // Each rival's deck builds a valid duel as the opponent seat.
        let human = arcana_cards::sample_deck(reg, 3);
        let p = &roster[0];
        let cfg = MatchConfig {
            seats: vec![
                SeatSpec::Local { profile: PlayerProfile::default(),
                    deck: human.clone(), identity: DeckIdentity::default() },
                SeatSpec::Bot { profile: p.profile.clone(), agenda: p.agenda.clone(),
                    deck: p.deck.clone(), identity: p.identity.clone(),
                    difficulty: p.difficulty },
            ],
            ..Default::default()
        };
        assert!(GameCore::from_match_config(reg, &cfg).is_ok(),
            "a roster rival is a valid opponent seat");
    }

    /// The catalog query layer works over the real ~20k-card catalog: an
    /// unconstrained query sees every card, and filters genuinely narrow it.
    #[test]
    fn catalog_query_over_real_catalog() {
        use arcana_core::catalog::{query, CardQuery};
        let reg = leaked_catalog();
        let all = query(reg, &CardQuery::default());
        assert_eq!(all.len(), reg.len(), "unconstrained query returns the whole catalog");
        assert!(all.len() > 1000, "the real catalog is large");
        assert!(all.iter().all(|c| !c.name.is_empty() && !c.type_line.is_empty()));

        let creatures = query(reg, &CardQuery {
            types: Some(vec!["creature".into()]), ..Default::default() });
        assert!(!creatures.is_empty() && creatures.len() < all.len());
        assert!(creatures.iter().all(|c| c.is_creature));

        let cheap_red = query(reg, &CardQuery {
            colors: Some(vec!['R']), cmc_max: Some(2), ..Default::default() });
        assert!(cheap_red.iter().all(|c| c.colors.contains(&'R') && c.mana_value <= 2));

        let capped = query(reg, &CardQuery { limit: Some(25), ..Default::default() });
        assert_eq!(capped.len(), 25);
    }

    /// The StateResponse round-trips through JSON — the actual web transport.
    #[test]
    fn state_response_serializes() {
        let reg = leaked_catalog();
        let mut core = GameCore::new(reg, 3);
        let s = core.snapshot();
        let json = serde_json::to_string(&s).expect("serialize StateResponse");
        let back: StateResponse = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, s);
    }

    // ---- Phase 1: seat-aware core (networked-duel foundation) --------------

    /// A two-HUMAN game on one `GameCore` (no bot) — the shape a networked duel
    /// runs. Tests reach into the private fields directly (child module).
    fn two_human_core(reg: &'static CardRegistry, seed: u64) -> GameCore {
        let deck = arcana_cards::sample_deck(reg, DECK_SEED);
        let seats = vec![Seat::Human, Seat::Human];
        let session = Session::new(vec![deck.clone(), deck], reg, seats, seed);
        GameCore { reg, session, legal: Vec::new(), awaiting: None }
    }

    /// With two human seats sharing one game, each seat's snapshot is from ITS
    /// OWN perspective: the seat to act sees its hand + legal actions; the other
    /// sees its own hand (opponent's hidden) and NO actions ("waiting").
    #[test]
    fn two_human_seats_each_get_their_own_perspective() {
        let reg = leaked_catalog();
        let mut core = two_human_core(reg, 7);

        // Populate `awaiting` and learn who acts first.
        core.snapshot_for(0);
        let actor = core.awaiting_seat().expect("someone must be to act");
        let waiter = 1 - actor;

        let s_actor = core.snapshot_for(actor);
        let s_wait = core.snapshot_for(waiter);

        // Perspective is each seat's own.
        assert_eq!(s_actor.view.perspective, actor);
        assert_eq!(s_wait.view.perspective, waiter);

        // Only the acting seat is given a decision.
        assert!(!s_actor.view.legal.is_empty(), "actor faces a real decision");
        assert!(s_wait.view.legal.is_empty(), "the waiting seat has no actions");

        // Each sees its own hand; the opponent's is hidden in both views.
        let a = actor as usize;
        let w = waiter as usize;
        assert_eq!(s_actor.view.players[a].hand.len(), s_actor.view.players[a].hand_count,
            "actor sees its own hand");
        assert!(s_actor.view.players[w].hand.is_empty(), "opponent hand hidden from actor");
        assert_eq!(s_wait.view.players[w].hand.len(), s_wait.view.players[w].hand_count,
            "waiter sees its own hand");
        assert!(s_wait.view.players[a].hand.is_empty(), "opponent hand hidden from waiter");
    }

    /// A seat may only act on its own turn: the off-turn seat is rejected with
    /// `NotYourTurn` (never mutating the game), while the acting seat succeeds.
    #[test]
    fn acting_out_of_turn_is_rejected() {
        let reg = leaked_catalog();
        let mut core = two_human_core(reg, 7);
        core.snapshot_for(0);
        let actor = core.awaiting_seat().expect("someone must be to act");
        let waiter = 1 - actor;

        assert_eq!(
            core.apply_index_for(waiter, 0),
            Err(ApplyError::NotYourTurn { seat: waiter, awaiting: Some(actor) }),
            "the off-turn seat can't act");
        // The actor can act, and after it does the turn passes to the other seat.
        assert!(core.apply_index_for(actor, 0).is_ok(), "the on-turn seat can act");
    }
}
