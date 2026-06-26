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
    /// Computed power/toughness; `None` for non-creatures.
    pub power: Option<i32>,
    pub toughness: Option<i32>,
    pub tapped: bool,
}

/// One player's public state plus (for the perspective player) their hand.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerView {
    pub id: PlayerId,
    pub life: i32,
    pub hand_count: usize,
    pub library_count: usize,
    pub graveyard_count: usize,
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

fn card_view(state: &GameState, registry: &CardRegistry, id: ObjectId) -> CardView {
    let name = state.objects.get(id)
        .and_then(|o| registry.interner().resolve(o.characteristics.name))
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .unwrap_or_default();
    let is_creature = state.objects.get(id)
        .map(|o| o.characteristics.types.is_creature()).unwrap_or(false);
    let (power, toughness) = if is_creature {
        (state.computed_power(id), state.computed_toughness(id))
    } else {
        (None, None)
    };
    let tapped = state.objects.get(id).map(|o| o.is_tapped()).unwrap_or(false);
    CardView { id, name, power, toughness, tapped }
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

    let players = (0..state.num_players()).map(|p| {
        PlayerView {
            id: p,
            life: state.player(p).life,
            hand_count: state.objects.objects_in_zone(Zone::Hand(p)).count(),
            library_count: state.objects.objects_in_zone(Zone::Library(p)).count(),
            graveyard_count: state.objects.objects_in_zone(Zone::Graveyard(p)).count(),
            hand: if p == perspective { cards_in(Zone::Hand(p)) } else { Vec::new() },
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
