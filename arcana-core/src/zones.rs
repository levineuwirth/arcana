//! Zone system: `Zone` enum and zone-taxonomy query helpers.
//!
//! Addendum Section 2, Phase 1 Task #3. Depends only on Task #1.
//!
//! CR 400.1: A zone is a place where objects can be during a game. There are
//! normally seven zones: library, hand, battlefield, graveyard, stack, exile,
//! and command. Each player has their own library, hand, and graveyard; the
//! other zones are shared (CR 400.2).

use serde::{Serialize, Deserialize};
use std::fmt;

use crate::types::PlayerId;

/// One of the seven standard MTG zones. `Library`, `Hand`, and `Graveyard`
/// carry their owner's `PlayerId`; the shared zones do not.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Zone {
    Library(PlayerId),
    Hand(PlayerId),
    Battlefield,
    Graveyard(PlayerId),
    Stack,
    Exile,
    Command,
}

impl Zone {
    /// Owner of this zone, if it's a per-player zone.
    pub const fn owner(self) -> Option<PlayerId> {
        match self {
            Zone::Library(p) | Zone::Hand(p) | Zone::Graveyard(p) => Some(p),
            Zone::Battlefield | Zone::Stack | Zone::Exile | Zone::Command => None,
        }
    }

    /// Per-player zone (CR 400.2): library, hand, graveyard.
    pub const fn is_personal(self) -> bool {
        matches!(self, Zone::Library(_) | Zone::Hand(_) | Zone::Graveyard(_))
    }

    /// Shared zone (CR 400.2): battlefield, stack, exile, command.
    pub const fn is_shared(self) -> bool { !self.is_personal() }

    /// Hidden zone — contents hidden from at least one player.
    /// Library is hidden from everyone (face-down); hand is hidden from
    /// opponents. All other zones are public.
    pub const fn is_hidden(self) -> bool {
        matches!(self, Zone::Library(_) | Zone::Hand(_))
    }

    /// Public zone — contents visible to all players.
    pub const fn is_public(self) -> bool { !self.is_hidden() }

    /// Ordered zone — position of objects matters. Only library (draw order)
    /// and stack (LIFO resolution) are ordered. Modern graveyard rules (CR
    /// 404.2) treat graveyards as unordered piles.
    pub const fn is_ordered(self) -> bool {
        matches!(self, Zone::Library(_) | Zone::Stack)
    }

    // Per-variant predicates — pattern matching ergonomics for callers.

    pub const fn is_library(self)     -> bool { matches!(self, Zone::Library(_)) }
    pub const fn is_hand(self)        -> bool { matches!(self, Zone::Hand(_)) }
    pub const fn is_graveyard(self)   -> bool { matches!(self, Zone::Graveyard(_)) }
    pub const fn is_battlefield(self) -> bool { matches!(self, Zone::Battlefield) }
    pub const fn is_stack(self)       -> bool { matches!(self, Zone::Stack) }
    pub const fn is_exile(self)       -> bool { matches!(self, Zone::Exile) }
    pub const fn is_command(self)     -> bool { matches!(self, Zone::Command) }

    /// Strip the player context — useful for filters like "any graveyard".
    pub const fn kind(self) -> ZoneKind {
        match self {
            Zone::Library(_)   => ZoneKind::Library,
            Zone::Hand(_)      => ZoneKind::Hand,
            Zone::Battlefield  => ZoneKind::Battlefield,
            Zone::Graveyard(_) => ZoneKind::Graveyard,
            Zone::Stack        => ZoneKind::Stack,
            Zone::Exile        => ZoneKind::Exile,
            Zone::Command      => ZoneKind::Command,
        }
    }

    /// True if this zone and `other` have the same kind, ignoring player.
    /// `Library(0).same_kind(Library(1))` is true; `Library(0).same_kind(Hand(0))`
    /// is false.
    pub const fn same_kind(self, other: Zone) -> bool {
        matches!(
            (self, other),
            (Zone::Library(_),    Zone::Library(_))
            | (Zone::Hand(_),       Zone::Hand(_))
            | (Zone::Graveyard(_),  Zone::Graveyard(_))
            | (Zone::Battlefield,   Zone::Battlefield)
            | (Zone::Stack,         Zone::Stack)
            | (Zone::Exile,         Zone::Exile)
            | (Zone::Command,       Zone::Command)
        )
    }

    /// Iterator over every zone relevant to an N-player game.
    /// For N players this yields `3 * N + 4` zones (3 personal zones per
    /// player plus 4 shared zones).
    pub fn all_for_n_players(n: u8) -> impl Iterator<Item = Zone> {
        let shared = [
            Zone::Battlefield, Zone::Stack, Zone::Exile, Zone::Command,
        ];
        let personal = (0..n).flat_map(|p| {
            [Zone::Library(p), Zone::Hand(p), Zone::Graveyard(p)]
        });
        personal.chain(shared)
    }
}

/// Zone identity without its player context. Useful for filters that apply
/// to zones of a given kind regardless of owner (e.g. "target card in any
/// graveyard").
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ZoneKind {
    Library,
    Hand,
    Battlefield,
    Graveyard,
    Stack,
    Exile,
    Command,
}

impl ZoneKind {
    /// Build a concrete `Zone` for a given player, if this kind is
    /// per-player. Returns `None` for shared zones.
    pub const fn with_player(self, p: PlayerId) -> Option<Zone> {
        match self {
            ZoneKind::Library    => Some(Zone::Library(p)),
            ZoneKind::Hand       => Some(Zone::Hand(p)),
            ZoneKind::Graveyard  => Some(Zone::Graveyard(p)),
            ZoneKind::Battlefield | ZoneKind::Stack
            | ZoneKind::Exile | ZoneKind::Command => None,
        }
    }

    /// Shared zones have a unique `Zone` value; return it directly.
    pub const fn as_shared_zone(self) -> Option<Zone> {
        match self {
            ZoneKind::Battlefield => Some(Zone::Battlefield),
            ZoneKind::Stack       => Some(Zone::Stack),
            ZoneKind::Exile       => Some(Zone::Exile),
            ZoneKind::Command     => Some(Zone::Command),
            _ => None,
        }
    }
}

impl fmt::Display for Zone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Zone::Library(p)   => write!(f, "Library(P{p})"),
            Zone::Hand(p)      => write!(f, "Hand(P{p})"),
            Zone::Battlefield  => f.write_str("Battlefield"),
            Zone::Graveyard(p) => write!(f, "Graveyard(P{p})"),
            Zone::Stack        => f.write_str("Stack"),
            Zone::Exile        => f.write_str("Exile"),
            Zone::Command      => f.write_str("Command"),
        }
    }
}

impl fmt::Display for ZoneKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            ZoneKind::Library     => "Library",
            ZoneKind::Hand        => "Hand",
            ZoneKind::Battlefield => "Battlefield",
            ZoneKind::Graveyard   => "Graveyard",
            ZoneKind::Stack       => "Stack",
            ZoneKind::Exile       => "Exile",
            ZoneKind::Command     => "Command",
        })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_of_personal_zones() {
        assert_eq!(Zone::Library(3).owner(),   Some(3));
        assert_eq!(Zone::Hand(0).owner(),      Some(0));
        assert_eq!(Zone::Graveyard(1).owner(), Some(1));
    }

    #[test]
    fn owner_of_shared_zones() {
        assert_eq!(Zone::Battlefield.owner(), None);
        assert_eq!(Zone::Stack.owner(),       None);
        assert_eq!(Zone::Exile.owner(),       None);
        assert_eq!(Zone::Command.owner(),     None);
    }

    #[test]
    fn personal_and_shared_partition() {
        for z in Zone::all_for_n_players(2) {
            // Every zone is exactly one of personal / shared.
            assert_ne!(z.is_personal(), z.is_shared(),
                "zone {z:?} reported as both / neither personal and shared");
        }
    }

    #[test]
    fn hidden_and_public_partition() {
        for z in Zone::all_for_n_players(2) {
            assert_ne!(z.is_hidden(), z.is_public(),
                "zone {z:?} reported as both / neither hidden and public");
        }
    }

    #[test]
    fn hidden_zones_are_library_and_hand() {
        assert!(Zone::Library(0).is_hidden());
        assert!(Zone::Hand(0).is_hidden());
        assert!(!Zone::Graveyard(0).is_hidden());
        assert!(!Zone::Battlefield.is_hidden());
        assert!(!Zone::Stack.is_hidden());
        assert!(!Zone::Exile.is_hidden());
        assert!(!Zone::Command.is_hidden());
    }

    #[test]
    fn ordered_zones_are_library_and_stack() {
        assert!(Zone::Library(0).is_ordered());
        assert!(Zone::Stack.is_ordered());
        // Everything else is unordered.
        assert!(!Zone::Hand(0).is_ordered());
        assert!(!Zone::Graveyard(0).is_ordered());
        assert!(!Zone::Battlefield.is_ordered());
        assert!(!Zone::Exile.is_ordered());
        assert!(!Zone::Command.is_ordered());
    }

    #[test]
    fn per_variant_predicates() {
        assert!(Zone::Library(0).is_library());
        assert!(Zone::Hand(0).is_hand());
        assert!(Zone::Graveyard(0).is_graveyard());
        assert!(Zone::Battlefield.is_battlefield());
        assert!(Zone::Stack.is_stack());
        assert!(Zone::Exile.is_exile());
        assert!(Zone::Command.is_command());

        // Negative check: hand is not library.
        assert!(!Zone::Hand(0).is_library());
    }

    #[test]
    fn kind_strips_player() {
        assert_eq!(Zone::Library(0).kind(),   ZoneKind::Library);
        assert_eq!(Zone::Library(5).kind(),   ZoneKind::Library);
        assert_eq!(Zone::Hand(2).kind(),      ZoneKind::Hand);
        assert_eq!(Zone::Graveyard(1).kind(), ZoneKind::Graveyard);
        assert_eq!(Zone::Battlefield.kind(),  ZoneKind::Battlefield);
        assert_eq!(Zone::Stack.kind(),        ZoneKind::Stack);
        assert_eq!(Zone::Exile.kind(),        ZoneKind::Exile);
        assert_eq!(Zone::Command.kind(),      ZoneKind::Command);
    }

    #[test]
    fn same_kind_ignores_player() {
        assert!(Zone::Library(0).same_kind(Zone::Library(1)));
        assert!(Zone::Hand(0).same_kind(Zone::Hand(1)));
        assert!(Zone::Graveyard(0).same_kind(Zone::Graveyard(1)));
        assert!(Zone::Battlefield.same_kind(Zone::Battlefield));

        assert!(!Zone::Library(0).same_kind(Zone::Hand(0)));
        assert!(!Zone::Battlefield.same_kind(Zone::Graveyard(0)));
    }

    #[test]
    fn zonekind_with_player_roundtrip() {
        assert_eq!(ZoneKind::Library.with_player(3),   Some(Zone::Library(3)));
        assert_eq!(ZoneKind::Hand.with_player(0),      Some(Zone::Hand(0)));
        assert_eq!(ZoneKind::Graveyard.with_player(1), Some(Zone::Graveyard(1)));
        // Shared zones return None
        assert_eq!(ZoneKind::Battlefield.with_player(0), None);
        assert_eq!(ZoneKind::Stack.with_player(0),       None);
    }

    #[test]
    fn zonekind_as_shared_zone() {
        assert_eq!(ZoneKind::Battlefield.as_shared_zone(), Some(Zone::Battlefield));
        assert_eq!(ZoneKind::Stack.as_shared_zone(),       Some(Zone::Stack));
        assert_eq!(ZoneKind::Exile.as_shared_zone(),       Some(Zone::Exile));
        assert_eq!(ZoneKind::Command.as_shared_zone(),     Some(Zone::Command));
        // Personal zones return None
        assert_eq!(ZoneKind::Library.as_shared_zone(),     None);
        assert_eq!(ZoneKind::Hand.as_shared_zone(),        None);
        assert_eq!(ZoneKind::Graveyard.as_shared_zone(),   None);
    }

    #[test]
    fn all_for_n_players_enumerates_correctly() {
        let zones: Vec<_> = Zone::all_for_n_players(2).collect();
        // 2 players × 3 personal + 4 shared = 10
        assert_eq!(zones.len(), 10);
        // No duplicates
        let unique: crate::collections::HashSet<_> = zones.iter().collect();
        assert_eq!(unique.len(), 10);
        // Contains expected entries
        assert!(zones.contains(&Zone::Library(0)));
        assert!(zones.contains(&Zone::Library(1)));
        assert!(zones.contains(&Zone::Battlefield));
        assert!(zones.contains(&Zone::Command));
    }

    #[test]
    fn all_for_n_players_scales() {
        assert_eq!(Zone::all_for_n_players(4).count(), 4 * 3 + 4);
    }

    #[test]
    fn display_formats() {
        assert_eq!(Zone::Library(0).to_string(),   "Library(P0)");
        assert_eq!(Zone::Hand(3).to_string(),      "Hand(P3)");
        assert_eq!(Zone::Graveyard(1).to_string(), "Graveyard(P1)");
        assert_eq!(Zone::Battlefield.to_string(),  "Battlefield");
        assert_eq!(Zone::Stack.to_string(),        "Stack");
        assert_eq!(Zone::Exile.to_string(),        "Exile");
        assert_eq!(Zone::Command.to_string(),      "Command");

        assert_eq!(ZoneKind::Library.to_string(),     "Library");
        assert_eq!(ZoneKind::Battlefield.to_string(), "Battlefield");
    }
}
