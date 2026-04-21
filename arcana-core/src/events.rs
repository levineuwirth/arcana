//! `GameEvent` taxonomy — the backbone of triggers, replacement effects,
//! and replay.
//!
//! Addendum Section 3 / Listing 9, Phase 1 Task #5. Depends on tasks 1, 3, 4.
//!
//! Every observable game action emits one or more [`GameEvent`]s. The
//! engine's trigger matcher walks the event stream; replacement effects
//! intercept it before the stream is committed; the event log is the
//! primary replay mechanism.

use serde::{Serialize, Deserialize};

use crate::combat::{AttackerDeclaration, BlockerDeclaration, DefendingEntity};
use crate::objects::ObjectId;
use crate::targets::TargetSelection;
use crate::turn::{Phase, Step};
use crate::types::*;
use crate::zones::Zone;

// =============================================================================
// GameEvent
// =============================================================================

#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum GameEvent {
    // === Zone transitions ===
    /// Fired on every zone change. Per CR 400.7 an object becomes a new
    /// object with a new id (`new_id`) when it changes zones; callers that
    /// care about post-move state should use `new_id`.
    ZoneChange {
        object_id: ObjectId,
        from: Zone,
        to: Zone,
        new_id: ObjectId,
        cause: MoveCause,
    },

    // === Specific zone-change semantic events ===
    // These fire after `ZoneChange` when applicable. Triggers usually
    // want these rather than the raw `ZoneChange`.
    EntersBattlefield {
        object_id: ObjectId,
        from_zone: Zone,
        was_cast: bool,
    },
    LeavesBattlefield {
        object_id: ObjectId,
        destination: Zone,
    },
    /// Creature or planeswalker died (battlefield → graveyard).
    Dies { object_id: ObjectId },
    PutIntoGraveyard { object_id: ObjectId, from: Zone },
    Exiled { object_id: ObjectId, from: Zone },
    DrawCard { player: PlayerId, object_id: ObjectId },
    Discarded { player: PlayerId, object_id: ObjectId },
    /// Library → graveyard (not cast or discarded).
    Milled { player: PlayerId, object_id: ObjectId },

    // === Spells and abilities ===
    SpellCast {
        object_id: ObjectId, // stack-entry id
        card_id: CardId,
        controller: PlayerId,
        targets: TargetSelection,
    },
    AbilityActivated {
        source: ObjectId,
        ability_index: usize,
        controller: PlayerId,
    },
    AbilityTriggered {
        source: ObjectId,
        trigger_id: TriggerId,
        controller: PlayerId,
    },
    SpellResolved { object_id: ObjectId },
    AbilityResolved { object_id: ObjectId },
    SpellCountered { object_id: ObjectId },
    /// CR 115 — `target` became the target of a spell or activated
    /// ability. Fires once per targeted object per source, after the
    /// spell/ability has landed on the stack with its targets set.
    /// The hook Ward (CR 702.21a) and other "whenever ~ becomes the
    /// target of ..." triggers attach to.
    ///
    /// `source` is the stack-entry id of the targeting spell or
    /// ability; `controller` is its caster/activator. Emitted by
    /// [`crate::engine::apply_cast_spell`] for spell casts, by
    /// the activated-ability dispatch for activations, and by the
    /// `ApplyTargetsToStackEntry` follow-up for triggered abilities
    /// that declared targets as they went on the stack
    /// (CR 603.3b).
    BecomesTarget {
        target: ObjectId,
        source: ObjectId,
        controller: PlayerId,
    },

    // === Damage ===
    DamageDealt {
        source: ObjectId,
        target: DamageTarget,
        amount: u32,
        is_combat: bool,
    },

    // === Life changes ===
    LifeGained { player: PlayerId, amount: u32 },
    LifeLost { player: PlayerId, amount: u32 },
    LifeSet { player: PlayerId, old: i32, new_total: i32 },

    // === Combat ===
    AttacksDeclared { attackers: Vec<AttackerDeclaration> },
    BlocksDeclared { blockers: Vec<BlockerDeclaration> },
    CreatureAttacks { attacker: ObjectId, defending: DefendingEntity },
    CreatureBlocks { blocker: ObjectId, attacker: ObjectId },
    CreatureBlocked { attacker: ObjectId, blockers: Vec<ObjectId> },
    CreatureNotBlocked { attacker: ObjectId },

    // === Permanent state changes ===
    Tapped { object_id: ObjectId },
    Untapped { object_id: ObjectId },
    Transformed { object_id: ObjectId },
    CounterAdded { object_id: ObjectId, kind: CounterKind, count: u32 },
    CounterRemoved { object_id: ObjectId, kind: CounterKind, count: u32 },
    AttachedTo { equipment_or_aura: ObjectId, target: ObjectId },
    Detached { equipment_or_aura: ObjectId, from: ObjectId },
    ControlChanged { object_id: ObjectId, old: PlayerId, new_ctrl: PlayerId },

    // === Turn/phase ===
    TurnBegins { player: PlayerId, turn_number: u32 },
    PhaseBegins { phase: Phase },
    StepBegins { step: Step },
    TurnEnds { player: PlayerId },

    // === Tokens and copies ===
    TokenCreated { object_id: ObjectId, controller: PlayerId },
    CopyCreated { object_id: ObjectId, copying: ObjectId },

    // === Player events ===
    PlayerLoses { player: PlayerId, reason: LoseReason },
    PlayerWins { player: PlayerId },
    ManaAdded { player: PlayerId, color: ManaColor, amount: u32 },
    Sacrifice { player: PlayerId, object_id: ObjectId },
    /// A player searched a library. `searching_player` is who actually
    /// performed the search (what "you" refers to in most card text);
    /// `library_owner` is whose library was searched. We intentionally
    /// don't expose this on [`GameEvent::affected_player`] — triggers that
    /// care should pattern-match on both fields explicitly.
    SearchedLibrary { searching_player: PlayerId, library_owner: PlayerId },
    Scry { player: PlayerId, count: u32 },
    Surveil { player: PlayerId, count: u32 },
    Explore { object_id: ObjectId },
    LibraryShuffled { player: PlayerId },
}

// =============================================================================
// EventKind — coarse categorization for fast trigger dispatch
// =============================================================================

/// Coarse category for a [`GameEvent`]. Triggers can query this to quickly
/// skip events they can't possibly care about, before doing finer
/// per-variant matching.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EventKind {
    /// `ZoneChange` and its refinements (ETB, LTB, Dies, Milled, …).
    ZoneTransition,
    /// Spell/ability lifecycle (cast, activate, trigger, resolve, counter).
    SpellOrAbility,
    /// `DamageDealt` only.
    Damage,
    /// `LifeGained`, `LifeLost`, `LifeSet`.
    Life,
    /// Combat declarations and block relationships.
    Combat,
    /// Tap/untap/transform/counter/attach/control.
    PermanentState,
    /// Turn, phase, and step boundaries.
    TurnPhase,
    /// `TokenCreated`, `CopyCreated`.
    TokenOrCopy,
    /// Player-centric events (win/lose, mana added, sacrifice, search, …).
    Player,
}

// =============================================================================
// Supporting enums
// =============================================================================

/// Why an object moved zones. Carried on `ZoneChange` for replacement
/// effects and trigger conditions that care ("when ~ dies from combat
/// damage").
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum MoveCause {
    SpellResolution,
    AbilityResolution,
    StateBasedAction,
    Cost,
    CombatDamage,
    /// Moved to the stack as part of casting.
    Cast,
    PlayLand,
    /// Drawn from a library.
    Draw,
    /// A replacement effect from this source redirected the move.
    Replacement(ObjectId),
}

/// Why a player lost the game. Used on `PlayerLoses`.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoseReason {
    /// Life total 0 or less (CR 704.5a).
    LifeZero,
    /// Drew from an empty library (CR 704.5b).
    Decked,
    /// Ten or more poison counters (CR 704.5c).
    PoisonCounters,
    /// A card effect ("you lose the game"). Records the source.
    CardEffect(ObjectId),
    Concession,
}

/// Something damage can be dealt to. Used on `DamageDealt` and by the
/// [`crate::effects::Effect::DealDamage`] primitive.
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum DamageTarget {
    Object(ObjectId),
    Player(PlayerId),
}

// =============================================================================
// Helper methods
// =============================================================================

impl GameEvent {
    /// Coarse category of this event. Useful as a fast prefilter in trigger
    /// dispatch before doing per-variant matching.
    pub fn kind(&self) -> EventKind {
        use GameEvent::*;
        use EventKind::*;
        match self {
            ZoneChange { .. }
            | EntersBattlefield { .. }
            | LeavesBattlefield { .. }
            | Dies { .. }
            | PutIntoGraveyard { .. }
            | Exiled { .. }
            | DrawCard { .. }
            | Discarded { .. }
            | Milled { .. } => ZoneTransition,

            SpellCast { .. }
            | AbilityActivated { .. }
            | AbilityTriggered { .. }
            | SpellResolved { .. }
            | AbilityResolved { .. }
            | SpellCountered { .. }
            | BecomesTarget { .. } => SpellOrAbility,

            DamageDealt { .. } => Damage,

            LifeGained { .. }
            | LifeLost { .. }
            | LifeSet { .. } => Life,

            AttacksDeclared { .. }
            | BlocksDeclared { .. }
            | CreatureAttacks { .. }
            | CreatureBlocks { .. }
            | CreatureBlocked { .. }
            | CreatureNotBlocked { .. } => Combat,

            Tapped { .. }
            | Untapped { .. }
            | Transformed { .. }
            | CounterAdded { .. }
            | CounterRemoved { .. }
            | AttachedTo { .. }
            | Detached { .. }
            | ControlChanged { .. } => PermanentState,

            TurnBegins { .. }
            | PhaseBegins { .. }
            | StepBegins { .. }
            | TurnEnds { .. } => TurnPhase,

            TokenCreated { .. }
            | CopyCreated { .. } => TokenOrCopy,

            PlayerLoses { .. }
            | PlayerWins { .. }
            | ManaAdded { .. }
            | Sacrifice { .. }
            | SearchedLibrary { .. }
            | Scry { .. }
            | Surveil { .. }
            | Explore { .. }
            | LibraryShuffled { .. } => Player,
        }
    }

    /// The primary "subject" object of this event — usually the object a
    /// `SelfX` trigger would check against. Events without a single subject
    /// (e.g. `PhaseBegins`, `AttacksDeclared`) return `None`.
    ///
    /// Note: for [`GameEvent::ZoneChange`] this returns the pre-move
    /// `object_id`, which is what `LeavesZone` triggers need. For
    /// `EntersBattlefield` etc. we already refer to the post-move id
    /// there.
    pub fn subject(&self) -> Option<ObjectId> {
        use GameEvent::*;
        Some(match self {
            ZoneChange { object_id, .. }
            | EntersBattlefield { object_id, .. }
            | LeavesBattlefield { object_id, .. }
            | Dies { object_id }
            | PutIntoGraveyard { object_id, .. }
            | Exiled { object_id, .. }
            | DrawCard { object_id, .. }
            | Discarded { object_id, .. }
            | Milled { object_id, .. }
            | SpellCast { object_id, .. }
            | SpellResolved { object_id }
            | AbilityResolved { object_id }
            | SpellCountered { object_id }
            | Tapped { object_id }
            | Untapped { object_id }
            | Transformed { object_id }
            | CounterAdded { object_id, .. }
            | CounterRemoved { object_id, .. }
            | ControlChanged { object_id, .. }
            | TokenCreated { object_id, .. }
            | Sacrifice { object_id, .. }
            | Explore { object_id } => *object_id,

            AbilityActivated { source, .. }
            | AbilityTriggered { source, .. }
            | DamageDealt { source, .. } => *source,

            CreatureAttacks { attacker, .. }
            | CreatureBlocked { attacker, .. }
            | CreatureNotBlocked { attacker } => *attacker,

            CreatureBlocks { blocker, .. } => *blocker,

            AttachedTo { equipment_or_aura, .. }
            | Detached { equipment_or_aura, .. } => *equipment_or_aura,

            CopyCreated { object_id, .. } => *object_id,

            // Becoming a target: the object becoming targeted is
            // the subject (what "Self becomes target" triggers check).
            BecomesTarget { target, .. } => *target,

            // Events with no single subject object
            LifeGained { .. } | LifeLost { .. } | LifeSet { .. }
            | AttacksDeclared { .. } | BlocksDeclared { .. }
            | TurnBegins { .. } | PhaseBegins { .. }
            | StepBegins { .. } | TurnEnds { .. }
            | PlayerLoses { .. } | PlayerWins { .. }
            | ManaAdded { .. } | SearchedLibrary { .. }
            | Scry { .. } | Surveil { .. }
            | LibraryShuffled { .. } => return None,
        })
    }

    /// Player principally affected by this event, if any.
    pub fn affected_player(&self) -> Option<PlayerId> {
        use GameEvent::*;
        Some(match self {
            DrawCard { player, .. }
            | Discarded { player, .. }
            | Milled { player, .. }
            | LifeGained { player, .. }
            | LifeLost { player, .. }
            | LifeSet { player, .. }
            | TurnBegins { player, .. }
            | TurnEnds { player }
            | PlayerLoses { player, .. }
            | PlayerWins { player }
            | ManaAdded { player, .. }
            | Sacrifice { player, .. }
            | Scry { player, .. }
            | Surveil { player, .. }
            | LibraryShuffled { player } => *player,

            SpellCast { controller, .. }
            | AbilityActivated { controller, .. }
            | AbilityTriggered { controller, .. }
            | BecomesTarget { controller, .. } => *controller,

            // SearchedLibrary deliberately does NOT map to a single
            // affected_player — callers must pattern-match on
            // `searching_player` vs `library_owner` explicitly. See the
            // variant's doc comment for rationale.

            // Events without a principal player
            _ => return None,
        })
    }

    // --- Category predicates for readability --------------------------------

    pub fn is_zone_transition(&self) -> bool { self.kind() == EventKind::ZoneTransition }
    pub fn is_damage(&self)          -> bool { self.kind() == EventKind::Damage }
    pub fn is_life_change(&self)     -> bool { self.kind() == EventKind::Life }
    pub fn is_combat_event(&self)    -> bool { self.kind() == EventKind::Combat }
    pub fn is_turn_phase(&self)      -> bool { self.kind() == EventKind::TurnPhase }

    /// True for `DamageDealt { is_combat: true }`. Used by CR 510.x triggers
    /// like "whenever a creature deals combat damage to a player".
    pub fn is_combat_damage(&self) -> bool {
        matches!(self, GameEvent::DamageDealt { is_combat: true, .. })
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::targets::TargetSelection;

    fn etb(id: ObjectId) -> GameEvent {
        GameEvent::EntersBattlefield {
            object_id: id,
            from_zone: Zone::Hand(0),
            was_cast: true,
        }
    }

    fn dies(id: ObjectId) -> GameEvent {
        GameEvent::Dies { object_id: id }
    }

    fn damage(src: ObjectId, target: ObjectId, amount: u32, combat: bool) -> GameEvent {
        GameEvent::DamageDealt {
            source: src,
            target: DamageTarget::Object(target),
            amount,
            is_combat: combat,
        }
    }

    // --- kind() categorization ------------------------------------------------

    #[test]
    fn kind_maps_zone_events() {
        assert_eq!(etb(1).kind(), EventKind::ZoneTransition);
        assert_eq!(dies(1).kind(), EventKind::ZoneTransition);
        let zc = GameEvent::ZoneChange {
            object_id: 1,
            from: Zone::Hand(0),
            to: Zone::Battlefield,
            new_id: 2,
            cause: MoveCause::SpellResolution,
        };
        assert_eq!(zc.kind(), EventKind::ZoneTransition);
    }

    #[test]
    fn kind_maps_damage_and_life() {
        assert_eq!(damage(1, 2, 3, true).kind(), EventKind::Damage);
        assert_eq!(
            GameEvent::LifeGained { player: 0, amount: 3 }.kind(),
            EventKind::Life
        );
        assert_eq!(
            GameEvent::LifeSet { player: 0, old: 20, new_total: 30 }.kind(),
            EventKind::Life
        );
    }

    #[test]
    fn kind_maps_combat() {
        let ev = GameEvent::CreatureAttacks {
            attacker: 1,
            defending: DefendingEntity::Player(1),
        };
        assert_eq!(ev.kind(), EventKind::Combat);
        let ev = GameEvent::AttacksDeclared { attackers: vec![] };
        assert_eq!(ev.kind(), EventKind::Combat);
    }

    #[test]
    fn kind_maps_spells_and_abilities() {
        let ev = GameEvent::SpellCast {
            object_id: 1,
            card_id: 42,
            controller: 0,
            targets: TargetSelection { targets: vec![] },
        };
        assert_eq!(ev.kind(), EventKind::SpellOrAbility);

        let ev = GameEvent::AbilityResolved { object_id: 1 };
        assert_eq!(ev.kind(), EventKind::SpellOrAbility);
    }

    #[test]
    fn kind_maps_permanent_state() {
        assert_eq!(GameEvent::Tapped { object_id: 1 }.kind(), EventKind::PermanentState);
        assert_eq!(
            GameEvent::CounterAdded {
                object_id: 1,
                kind: CounterKind::PlusOnePlusOne,
                count: 2,
            }.kind(),
            EventKind::PermanentState
        );
    }

    #[test]
    fn kind_maps_turn_phase() {
        assert_eq!(
            GameEvent::TurnBegins { player: 0, turn_number: 1 }.kind(),
            EventKind::TurnPhase
        );
        assert_eq!(
            GameEvent::PhaseBegins { phase: Phase::Combat }.kind(),
            EventKind::TurnPhase
        );
        assert_eq!(
            GameEvent::StepBegins { step: Step::Upkeep }.kind(),
            EventKind::TurnPhase
        );
    }

    #[test]
    fn kind_maps_player_events() {
        assert_eq!(
            GameEvent::PlayerLoses { player: 0, reason: LoseReason::LifeZero }.kind(),
            EventKind::Player
        );
        assert_eq!(
            GameEvent::ManaAdded { player: 0, color: ManaColor::Red, amount: 2 }.kind(),
            EventKind::Player
        );
    }

    // --- subject() extraction -------------------------------------------------

    #[test]
    fn subject_of_zone_events() {
        assert_eq!(etb(42).subject(), Some(42));
        assert_eq!(dies(42).subject(), Some(42));
    }

    #[test]
    fn subject_of_damage_is_source() {
        assert_eq!(damage(5, 10, 3, false).subject(), Some(5));
    }

    #[test]
    fn subject_of_combat_attack_is_attacker() {
        let ev = GameEvent::CreatureAttacks {
            attacker: 42,
            defending: DefendingEntity::Player(1),
        };
        assert_eq!(ev.subject(), Some(42));

        let ev = GameEvent::CreatureBlocks { blocker: 7, attacker: 8 };
        assert_eq!(ev.subject(), Some(7));
    }

    #[test]
    fn subject_of_attacks_declared_is_none() {
        // AttacksDeclared is the aggregate event; there's no single subject.
        let ev = GameEvent::AttacksDeclared { attackers: vec![] };
        assert_eq!(ev.subject(), None);
    }

    #[test]
    fn subject_of_phase_events_is_none() {
        assert_eq!(GameEvent::PhaseBegins { phase: Phase::Combat }.subject(), None);
        assert_eq!(GameEvent::TurnEnds { player: 0 }.subject(), None);
    }

    #[test]
    fn subject_of_life_events_is_none() {
        assert_eq!(GameEvent::LifeGained { player: 0, amount: 3 }.subject(), None);
    }

    // --- affected_player() ---------------------------------------------------

    #[test]
    fn affected_player_of_draw() {
        let ev = GameEvent::DrawCard { player: 1, object_id: 42 };
        assert_eq!(ev.affected_player(), Some(1));
    }

    #[test]
    fn affected_player_of_life_change() {
        assert_eq!(
            GameEvent::LifeLost { player: 0, amount: 3 }.affected_player(),
            Some(0)
        );
    }

    #[test]
    fn affected_player_of_spell_cast_is_controller() {
        let ev = GameEvent::SpellCast {
            object_id: 1,
            card_id: 42,
            controller: 1,
            targets: TargetSelection { targets: vec![] },
        };
        assert_eq!(ev.affected_player(), Some(1));
    }

    #[test]
    fn searched_library_has_no_single_affected_player() {
        // Fields are split: searching_player + library_owner. affected_player()
        // deliberately returns None — callers must pattern-match explicitly.
        let ev = GameEvent::SearchedLibrary {
            searching_player: 1,
            library_owner: 0,
        };
        assert_eq!(ev.affected_player(), None);
    }

    #[test]
    fn searched_library_fields_are_both_accessible() {
        // The whole point of splitting is that both players are named.
        let ev = GameEvent::SearchedLibrary {
            searching_player: 1,
            library_owner: 0,
        };
        match ev {
            GameEvent::SearchedLibrary { searching_player, library_owner } => {
                assert_eq!(searching_player, 1);
                assert_eq!(library_owner, 0);
            }
            _ => unreachable!(),
        }
    }

    #[test]
    fn affected_player_of_damage_is_none() {
        // Damage doesn't have a single "affected player" in the general
        // case (the target might be an object). Callers who want the
        // damaged player should pattern-match on DamageTarget::Player.
        assert_eq!(damage(1, 2, 3, true).affected_player(), None);
    }

    // --- is_combat_damage ----------------------------------------------------

    #[test]
    fn is_combat_damage_only_combat() {
        assert!(damage(1, 2, 3, true).is_combat_damage());
        assert!(!damage(1, 2, 3, false).is_combat_damage());
        assert!(!GameEvent::LifeGained { player: 0, amount: 3 }.is_combat_damage());
    }

    // --- category predicates -------------------------------------------------

    #[test]
    fn category_predicates() {
        assert!(etb(1).is_zone_transition());
        assert!(damage(1, 2, 3, true).is_damage());
        assert!(GameEvent::LifeGained { player: 0, amount: 3 }.is_life_change());
        assert!(
            GameEvent::CreatureAttacks {
                attacker: 1,
                defending: DefendingEntity::Player(0),
            }
            .is_combat_event()
        );
        assert!(GameEvent::TurnBegins { player: 0, turn_number: 1 }.is_turn_phase());
    }

    // --- serde roundtrip -----------------------------------------------------

    #[test]
    fn events_serialize_and_deserialize() {
        // A reasonably complex event round-trips through JSON.
        let ev = GameEvent::DamageDealt {
            source: 42,
            target: DamageTarget::Player(1),
            amount: 7,
            is_combat: true,
        };
        let json = serde_json::to_string(&ev).expect("serialize");
        let back: GameEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(ev, back);
    }

    #[test]
    fn zone_change_serializes() {
        let ev = GameEvent::ZoneChange {
            object_id: 1,
            from: Zone::Hand(0),
            to: Zone::Battlefield,
            new_id: 2,
            cause: MoveCause::SpellResolution,
        };
        let json = serde_json::to_string(&ev).expect("serialize");
        let back: GameEvent = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(ev, back);
    }

    // --- equality is structural ---------------------------------------------

    #[test]
    fn events_equal_by_fields() {
        assert_eq!(dies(1), dies(1));
        assert_ne!(dies(1), dies(2));
        assert_ne!(dies(1), etb(1));
    }
}
