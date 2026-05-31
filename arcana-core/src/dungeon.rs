//! CR 309 / 701.49 — Venture into the Dungeon.
//!
//! A dungeon is a room graph the player advances through one room at a
//! time. Venturing while not in a dungeon enters a dungeon's first room;
//! venturing while in one advances to a room connected to the current
//! one. Entering a room fires its effect; entering a terminal room (no
//! outgoing edges) completes the dungeon, after which the next venture
//! starts fresh.
//!
//! The per-player POSITION (which dungeon, which room) is stored on
//! [`crate::state::PlayerState::dungeon`]; the room TABLE — including the
//! fn-pointer room effects — is the static data below, so nothing
//! unserializable lives on the game state.
//!
//! **Phase-1 scope**: the iconic *Lost Mine of Phandelver* is modeled in
//! full. The other official dungeons (Dungeon of the Mad Mage, Tomb of
//! Annihilation, Undercity) share this framework — add their room tables
//! to [`rooms`] to enable them; until then every venture uses Lost Mine.
//! Two documented approximations: (1) the branch choice at a room with
//! multiple exits is taken DETERMINISTICALLY (first edge) rather than by
//! a player prompt; (2) a couple of room effects that need mana/interner
//! access are approximated with the nearest clean effect.

use crate::effects::Effect;
use crate::types::PlayerId;

/// Which dungeon a player is venturing through. Stored on PlayerState
/// (plain enum — serializable); the room data lives in [`rooms`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DungeonId {
    LostMine,
    MadMage,
    TombOfAnnihilation,
    Undercity,
}

/// A player's current spot in a dungeon: which dungeon, and the index of
/// the room they're in within that dungeon's [`rooms`] table.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct DungeonPosition {
    pub dungeon: DungeonId,
    pub room: usize,
}

/// One room of a dungeon: a display name, the effect that fires on
/// entry (built fresh for the venturing `player`), and the indices of
/// the rooms reachable from here (empty = terminal → completes the
/// dungeon).
pub struct Room {
    pub name: &'static str,
    pub on_enter: fn(PlayerId) -> Vec<Effect>,
    pub next: &'static [usize],
}

// --- Lost Mine of Phandelver room effects --------------------------------
// Effects are kept to interner-free primitives; where the printed room
// needs mana/typed-token machinery the nearest clean effect is used (a
// documented Phase-1 approximation).

fn cave_entrance(p: PlayerId) -> Vec<Effect> { vec![Effect::Scry { player: p, count: 1 }] }
fn goblin_lair(p: PlayerId) -> Vec<Effect> {
    // "Create a 1/1 red Goblin." Minted as a bare 1/1 red token (no
    // subtype — Effect::execute has no interner; documented partial).
    vec![Effect::CreateToken {
        controller: p,
        token: crate::effects::TokenDefinition {
            name: 0,
            colors: crate::types::ColorSet::red(),
            types: crate::types::TypeLine::CREATURE.into(),
            subtypes: crate::types::SubtypeSet::default(),
            power: Some(crate::types::PtValue::Fixed(1)),
            toughness: Some(crate::types::PtValue::Fixed(1)),
            keywords: Vec::new(),
            abilities: Vec::new(),
        },
    }]
}
fn storeroom(p: PlayerId) -> Vec<Effect> { vec![Effect::DrawCards { player: p, count: 1 }] }
fn dark_pool(p: PlayerId) -> Vec<Effect> {
    vec![Effect::DrawCards { player: p, count: 1 }, Effect::LoseLife { player: p, amount: 1 }]
}
fn mine_tunnels(p: PlayerId) -> Vec<Effect> {
    // "Add one mana of any color." Approximated as Surveil 1 (a clean
    // tempo effect) pending mana-pool access from a room callback.
    vec![Effect::Surveil { player: p, count: 1 }]
}
fn temple_of_dumathoin(p: PlayerId) -> Vec<Effect> {
    // Terminal room — completing the dungeon. The printed reveal-and-
    // may-take is approximated with a draw.
    vec![Effect::DrawCards { player: p, count: 1 }]
}

static LOST_MINE: &[Room] = &[
    Room { name: "Cave Entrance",        on_enter: cave_entrance,       next: &[1, 2] },
    Room { name: "Goblin Lair",          on_enter: goblin_lair,         next: &[3, 2] },
    Room { name: "Dark Pool",            on_enter: dark_pool,           next: &[3, 4] },
    Room { name: "Storeroom",            on_enter: storeroom,           next: &[4, 5] },
    Room { name: "Mine Tunnels",         on_enter: mine_tunnels,        next: &[5] },
    Room { name: "Temple of Dumathoin",  on_enter: temple_of_dumathoin, next: &[] },
];

/// The room table for a dungeon. Phase-1: every dungeon resolves to Lost
/// Mine of Phandelver (the others are framework-ready — drop their
/// tables in here to enable them).
pub fn rooms(_dungeon: DungeonId) -> &'static [Room] {
    LOST_MINE
}

/// The dungeon a fresh venture enters. Phase-1 default — a player-choice
/// among dungeons is documented debt.
pub fn default_dungeon() -> DungeonId {
    DungeonId::LostMine
}
