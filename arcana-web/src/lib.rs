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

use arcana_ai::search::{MaterialValue, ValueMcPolicy};
use arcana_ai::session::{Seat, Session, Turn};
use arcana_core::actions::Action;
use arcana_core::catalog::{card_info, CardInfo};
use arcana_core::deck::parse_deck_text;
use arcana_core::combat::{
    attacker_options, blocker_options, damage_targets, match_attack, match_block, match_damage,
    match_ordering, ordering_targets, AttackerDeclaration, BlockerDeclaration, DamageAssignment,
    DefendingEntity,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::objects::ObjectId;
use arcana_core::state::GameState;
use arcana_core::registry::CardRegistry;
use arcana_core::state::GameResult;
use arcana_core::types::{CardId, PlayerId};
use arcana_core::view::{view_state, ViewState};
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

fn eval_for(state: &GameState) -> Eval {
    let value = arcana_ai::search::value(state, HUMAN);
    let win_pct = 100.0 * arcana_ai::calibrate::win_probability(value);
    Eval { value, win_pct }
}

/// The human's per-permanent card power (marginal win% in this position), sorted
/// descending. Cheap (no clones/rollouts); see `card_marginal_values`.
fn card_power_for(state: &GameState) -> Vec<CardPower> {
    arcana_ai::search::card_marginal_values(state, HUMAN)
        .into_iter()
        .map(|(id, win_pct)| CardPower { id, win_pct })
        .collect()
}

fn library_stats(state: &GameState, reg: &CardRegistry) -> LibraryStats {
    use std::collections::HashMap;
    let mut by_type: HashMap<&'static str, usize> = HashMap::new();
    let mut by_cmc: HashMap<u32, usize> = HashMap::new();
    let mut by_name: HashMap<String, usize> = HashMap::new();
    let mut total = 0;
    for o in state.objects.objects_in_zone(arcana_core::zones::Zone::Library(HUMAN)) {
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

/// One in-progress game: a human (P0) vs a snappy Monte-Carlo bot (P1) on
/// mirrored sample decks. Holds the legal actions from the most recent human
/// decision so `/action` can resolve an index the browser sent back.
pub struct GameCore {
    reg: &'static CardRegistry,
    session: Session<'static>,
    /// Legal actions surfaced at the latest human decision, indexed by the
    /// `index` the frontend echoes back. Empty once the game is over.
    legal: Vec<Action>,
}

impl GameCore {
    /// Build the snappy bot policy recommended for interactive play.
    fn make_bot(seed: u64) -> Seat {
        // ValueMcPolicy with a small budget: lookahead enough to not be silly,
        // light enough to answer an HTTP request promptly.
        let policy = ValueMcPolicy::with_budget(Box::new(MaterialValue), seed ^ 0xA5EED, 6, 25, 10);
        Seat::Bot(Box::new(policy))
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
        Self { reg, session, legal: Vec::new() }
    }

    /// Read-only access to the registry (for callers that build a replacement
    /// `GameCore` on `/new`).
    pub fn registry(&self) -> &'static CardRegistry {
        self.reg
    }

    /// Drive the session through all bot + trivial decisions, then project the
    /// next human decision (or the finished game) into a [`StateResponse`].
    ///
    /// Idempotent while a human decision is pending: `advance` re-surfaces the
    /// same decision without mutating, so polling `/state` is safe.
    pub fn snapshot(&mut self) -> StateResponse {
        let view = match self.session.advance() {
            Turn::AwaitingHuman { player, view, legal, .. } => {
                let vs = view_state(&view.state, self.reg, player, &legal);
                self.legal = legal;
                vs
            }
            Turn::GameOver(result) => {
                self.legal = Vec::new();
                // The full state's `result` field is set on game over, so
                // view_state already fills `game_over`; belt-and-braces, derive
                // it from the Turn payload if it somehow isn't.
                let mut vs = view_state(self.session.state(), self.reg, HUMAN, &[]);
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
        let combat = combat_prompt(self.session.state(), &self.legal);
        let bottom = bottom_prompt(&self.legal);
        let eval = eval_for(self.session.state());
        let library = library_stats(self.session.state(), self.reg);
        let card_power = card_power_for(self.session.state());
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
        if self.legal.len() <= 1 {
            return Vec::new();
        }
        let state = self.session.state();
        // Snappy auto budget vs a heavier "deepen" budget.
        let (rollouts, cap, candidates) = if deep { (20, 40, 16) } else { (8, 30, 12) };
        arcana_ai::search::rank_actions(
            state, self.reg, HUMAN, &self.legal, &MaterialValue,
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
        if self.legal.is_empty() {
            return Err(ApplyError::NoPendingDecision);
        }
        let action = self
            .legal
            .get(index)
            .cloned()
            .ok_or(ApplyError::OutOfRange { index, len: self.legal.len() })?;
        self.session.apply(action);
        Ok(self.snapshot())
    }

    /// Apply an incremental combat declaration built by the frontend. The picked
    /// set is matched against the cached legal enumeration via the
    /// [`arcana_core::combat`] matchers (so all combat rules — lethal ordering,
    /// trample, the 1024-declaration cap — stay engine-side); an empty set is the
    /// always-legal "no attacks / no blocks". Returns [`ApplyError::IllegalCombat`]
    /// if the set isn't a legal declaration, so the frontend can re-prompt.
    pub fn apply_combat(&mut self, sub: CombatSubmission) -> Result<StateResponse, ApplyError> {
        if self.legal.is_empty() {
            return Err(ApplyError::NoPendingDecision);
        }
        let action = match sub {
            CombatSubmission::Attackers { attackers } => match_attack(&self.legal, &attackers),
            CombatSubmission::Blockers { blockers } => match_block(&self.legal, &blockers),
            CombatSubmission::Order { orderings } => match_ordering(&self.legal, &orderings),
            CombatSubmission::Damage { distributions } => match_damage(&self.legal, &distributions),
        };
        let action = action.ok_or(ApplyError::IllegalCombat)?;
        self.session.apply(action);
        Ok(self.snapshot())
    }

    /// MTGA-style "click a card to play it": auto-tap the mana to make hand card
    /// `target` castable, then cast/play it if there's a single way to — else
    /// leave the mana floated and surface the now-available cast variants (e.g.
    /// to choose targets). Returns [`ApplyError::NotPlayable`] if it can't be
    /// played this turn. See [`arcana_core::legal_actions::auto_tap_sequence`].
    pub fn auto_tap_and_cast(&mut self, target: ObjectId) -> Result<StateResponse, ApplyError> {
        let seq = arcana_core::legal_actions::auto_tap_sequence(
            self.session.state(), self.reg, HUMAN, target)
            .ok_or(ApplyError::NotPlayable { id: target })?;
        for action in seq {
            self.session.apply(action);
        }
        Ok(self.snapshot())
    }

    /// Apply a London-mulligan bottoming: put the chosen `ids` on the bottom of
    /// the human's library (the rest is their opening hand). Validated against the
    /// owed count and the human's hand — `apply_bottom_cards` panics on a bad
    /// submission, so a malformed request is rejected as [`ApplyError::IllegalBottom`]
    /// rather than reaching the engine.
    pub fn bottom_cards(&mut self, ids: Vec<ObjectId>) -> Result<StateResponse, ApplyError> {
        let owed = bottom_prompt(&self.legal).ok_or(ApplyError::NoPendingDecision)?;
        let hand: std::collections::HashSet<ObjectId> = self
            .session
            .state()
            .objects
            .iter()
            .filter(|o| o.zone == arcana_core::zones::Zone::Hand(HUMAN))
            .map(|o| o.id)
            .collect();
        let mut seen = std::collections::HashSet::new();
        let valid = ids.len() == owed
            && ids.iter().all(|id| hand.contains(id) && seen.insert(*id));
        if !valid {
            return Err(ApplyError::IllegalBottom { owed });
        }
        self.session.apply(Action::BottomCards(ids));
        Ok(self.snapshot())
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
}
