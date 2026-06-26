//! Serializable view of a game for UI frontends — the keystone for the
//! web/server GUI (the live [`GameState`]/`Session` can't serialize because it
//! holds fn-pointer-bearing continuous/replacement/triggered effects, so we send
//! a flattened projection instead) and equally usable by a native egui app
//! reading it in-process.
//!
//! [`view_state`] projects `(GameState, perspective, legal actions)` into a
//! [`ViewState`] of plain serde data: life, zone counts, the perspective
//! player's hand, every battlefield permanent (name + computed P/T + tapped),
//! the stack, and the legal actions with human-readable labels. `name` is enough
//! for a frontend to fetch card art (e.g. from Scryfall). Hidden information is
//! respected — only the perspective player's hand contents are filled in; other
//! hands are counts only. Pass a perspective-projected state if you also want
//! opponents' hidden cards anonymized at the source.

use serde::{Deserialize, Serialize};

use crate::actions::Action;
use crate::objects::ObjectId;
use crate::registry::CardRegistry;
use crate::render::render_action;
use crate::state::{GameResult, GameState};
use crate::types::PlayerId;
use crate::zones::Zone;

/// One battlefield/hand/stack object, flattened for display.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct CardView {
    pub id: ObjectId,
    /// Card name — also the key a frontend uses to fetch art (Scryfall, etc.).
    /// Empty for a hidden/anonymized object.
    pub name: String,
    /// Rendered mana cost ("{2}{G}{G}"); `None` for objects with no cost (lands,
    /// tokens) or hidden objects.
    pub mana_cost: Option<String>,
    /// Mana value (converted mana cost) — 0 if no cost.
    pub mana_value: u32,
    /// Printed type line ("Legendary Creature — Elf Warrior", "Basic Land —
    /// Forest", "Instant"). Empty for a hidden object.
    pub type_line: String,
    /// Whether this object is a land — lets a frontend tally untapped mana
    /// sources without re-parsing the type line.
    pub is_land: bool,
    /// For the perspective player's hand only: true if this card could be cast
    /// or played this turn assuming the player taps out (see
    /// [`crate::legal_actions::playable_cards`]). Always false for opponents'
    /// objects, the battlefield, and the stack.
    pub playable: bool,
    /// Printed keyword abilities by stable glossary key ("Flying", "Ward", …),
    /// deduped + sorted. A frontend pairs these with
    /// [`crate::glossary::keyword_glossary`] for reminder text.
    pub keywords: Vec<String>,
    /// Computed power/toughness; `None` for non-creatures.
    pub power: Option<i32>,
    pub toughness: Option<i32>,
    pub tapped: bool,
}

/// A mana amount broken down by color — used for the "available mana" gauge.
/// `total == white + blue + black + red + green + colorless`.
#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct ManaCounts {
    pub white: usize,
    pub blue: usize,
    pub black: usize,
    pub red: usize,
    pub green: usize,
    pub colorless: usize,
    pub total: usize,
}

fn mana_counts(pool: &crate::mana::ManaPool) -> ManaCounts {
    use crate::types::ManaColor;
    let mut c = ManaCounts::default();
    for u in pool.iter() {
        match u.color {
            ManaColor::White => c.white += 1,
            ManaColor::Blue => c.blue += 1,
            ManaColor::Black => c.black += 1,
            ManaColor::Red => c.red += 1,
            ManaColor::Green => c.green += 1,
            ManaColor::Colorless => c.colorless += 1,
        }
        c.total += 1;
    }
    c
}

/// One player's public state plus (for the perspective player) their hand.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerView {
    pub id: PlayerId,
    pub life: i32,
    pub hand_count: usize,
    pub library_count: usize,
    pub graveyard_count: usize,
    /// Floating (unspent) mana in this player's pool — usually 0 between
    /// decisions, non-zero mid-cast.
    pub mana_pool: usize,
    /// Mana this player could produce right now (floating pool + everything
    /// their untapped mana abilities can still make), by color. Drives the
    /// "available mana" gauge. See [`crate::legal_actions::available_mana`] for
    /// the flexible-source caveat.
    pub available_mana: ManaCounts,
    /// Filled only for the perspective player (hidden information).
    pub hand: Vec<CardView>,
    pub battlefield: Vec<CardView>,
}

/// A legal action plus its display label and stable index into the `legal`
/// slice the frontend was given (the frontend sends the index back to apply).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActionView {
    pub index: usize,
    pub label: String,
}

/// A complete, serializable snapshot of the game from one player's view.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ViewState {
    pub turn: u32,
    /// "{phase:?}/{step:?}".
    pub phase: String,
    pub active_player: PlayerId,
    pub priority_player: PlayerId,
    /// Whose view this is.
    pub perspective: PlayerId,
    pub players: Vec<PlayerView>,
    pub stack: Vec<CardView>,
    pub legal: Vec<ActionView>,
    /// Set once the game is decided ("Win(0)" / "Draw" / …).
    pub game_over: Option<String>,
}

/// Build a printed type line: "{supertypes} {types} — {subtypes}" (the em-dash
/// and subtype clause only when subtypes are present). Subtypes are resolved via
/// the interner and sorted for deterministic display (the underlying set is
/// unordered).
fn type_line(c: &crate::objects::Characteristics, registry: &CardRegistry) -> String {
    use crate::types::TypeLine;
    let mut head: Vec<&str> = Vec::new();
    let s = &c.supertypes;
    if s.is_basic() { head.push("Basic"); }
    if s.is_legendary() { head.push("Legendary"); }
    if s.is_snow() { head.push("Snow"); }
    if s.is_world() { head.push("World"); }
    let t = &c.types;
    if t.is_artifact() { head.push("Artifact"); }
    if t.is_battle() { head.push("Battle"); }
    if t.is_creature() { head.push("Creature"); }
    if t.is_enchantment() { head.push("Enchantment"); }
    if t.is_instant() { head.push("Instant"); }
    if t.has(TypeLine::KINDRED) { head.push("Kindred"); }
    if t.is_land() { head.push("Land"); }
    if t.is_planeswalker() { head.push("Planeswalker"); }
    if t.is_sorcery() { head.push("Sorcery"); }

    let mut subs: Vec<&str> = c.subtypes.iter()
        .filter_map(|sm| registry.interner().resolve(sm))
        .collect();
    subs.sort_unstable();

    let head = head.join(" ");
    if subs.is_empty() { head } else { format!("{head} — {}", subs.join(" ")) }
}

fn card_view(state: &GameState, registry: &CardRegistry, id: ObjectId) -> CardView {
    let Some(o) = state.objects.get(id) else {
        return CardView {
            id, name: String::new(), mana_cost: None, mana_value: 0,
            type_line: String::new(), is_land: false, playable: false,
            keywords: Vec::new(), power: None, toughness: None, tapped: false,
        };
    };
    let c = &o.characteristics;
    let name = registry.interner().resolve(c.name)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_default();
    let mana_cost = c.mana_cost.as_ref().map(|mc| mc.to_string());
    let mana_value = c.mana_value();
    let type_line = type_line(c, registry);
    let is_land = c.types.is_land();
    let (power, toughness) = if c.types.is_creature() {
        (state.computed_power(id), state.computed_toughness(id))
    } else {
        (None, None)
    };
    let tapped = o.is_tapped();
    let mut keywords: Vec<String> = c.keywords.iter()
        .map(crate::glossary::keyword_key).collect();
    keywords.sort_unstable();
    keywords.dedup();
    CardView {
        id, name, mana_cost, mana_value, type_line, is_land,
        playable: false, keywords, power, toughness, tapped,
    }
}

/// Project `state` (from `perspective`'s view) plus its `legal` actions into a
/// [`ViewState`]. See module docs for the hidden-information contract.
pub fn view_state(
    state: &GameState,
    registry: &CardRegistry,
    perspective: PlayerId,
    legal: &[Action],
) -> ViewState {
    let cards_in = |zone: Zone| -> Vec<CardView> {
        let ids: Vec<ObjectId> = state.objects.objects_in_zone(zone).map(|o| o.id).collect();
        ids.into_iter().map(|id| card_view(state, registry, id)).collect()
    };

    // Which of the perspective player's hand cards are playable this turn (cast
    // or play if they tap out). Computed once; only that player's hand is shown.
    let playable: std::collections::HashSet<ObjectId> =
        crate::legal_actions::playable_cards(state, perspective, registry)
            .into_iter().collect();

    let players = (0..state.num_players()).map(|p| {
        PlayerView {
            id: p,
            life: state.player(p).life,
            hand_count: state.objects.objects_in_zone(Zone::Hand(p)).count(),
            library_count: state.objects.objects_in_zone(Zone::Library(p)).count(),
            graveyard_count: state.objects.objects_in_zone(Zone::Graveyard(p)).count(),
            mana_pool: state.player(p).mana_pool.total(),
            available_mana: mana_counts(&crate::legal_actions::available_mana(state, p, registry)),
            hand: if p == perspective {
                let mut h = cards_in(Zone::Hand(p));
                for c in &mut h { c.playable = playable.contains(&c.id); }
                h
            } else {
                Vec::new()
            },
            battlefield: {
                let ids: Vec<ObjectId> = state.objects.objects_in_zone(Zone::Battlefield)
                    .filter(|o| o.controller == p).map(|o| o.id).collect();
                ids.into_iter().map(|id| card_view(state, registry, id)).collect()
            },
        }
    }).collect();

    let stack = state.stack.iter()
        .filter_map(|e| e.card_id().map(|_| e.id))
        .map(|id| card_view(state, registry, id))
        .collect();

    let legal = legal.iter().enumerate()
        .map(|(index, a)| ActionView { index, label: render_action(a, state, registry) })
        .collect();

    let game_over = match state.result {
        Some(GameResult::Win(p)) => Some(format!("Win(P{p})")),
        Some(GameResult::Draw) => Some("Draw".to_string()),
        Some(GameResult::Eliminated(p)) => Some(format!("Eliminated(P{p})")),
        None => None,
    };

    ViewState {
        turn: state.turn.turn_number,
        phase: format!("{:?}/{:?}", state.turn.phase, state.turn.step),
        active_player: state.active_player(),
        priority_player: state.priority_player(),
        perspective,
        players,
        stack,
        legal,
        game_over,
    }
}
