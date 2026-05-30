//! Intervening-if condition prelude (CR 603.4).
//!
//! A small, **total, panic-free, read-only** set of boolean predicates
//! for [`crate::triggers::InterveningIfFn`] bodies — the sibling of
//! [`crate::script`] (which returns resolution-time *amounts*; these
//! return the *bool* a trigger's `intervening_if` needs).
//!
//! A generated card wires an intervening-if by writing a tiny named
//! `fn(&GameState, ObjectId, PlayerId) -> bool` (state, source,
//! controller) that calls one of these, then setting
//! `intervening_if: Some(that_fn)` on its [`crate::triggers::TriggeredAbilityDef`]:
//!
//! ```ignore
//! fn if_control_three_artifacts(s: &GameState, _src: ObjectId, you: PlayerId) -> bool {
//!     conditions::you_control_at_least(
//!         s, you,
//!         &ObjectFilter { types: Some(TypeLine::ARTIFACT.into()), ..Default::default() },
//!         3,
//!     )
//! }
//! ```
//!
//! Same defensive contract as [`crate::script`]: an invalid player or
//! missing object yields the neutral answer (`false` / `0`), never a
//! panic.

use crate::objects::ObjectId;
use crate::state::GameState;
use crate::targets::ObjectFilter;
use crate::types::{CounterKind, PlayerId};
use crate::zones::Zone;

fn valid(state: &GameState, p: PlayerId) -> bool {
    p < state.num_players()
}

/// Battlefield permanents **`you` control** matching `filter`. The
/// filter's [`crate::targets::ControllerConstraint::You`] (if any)
/// resolves against `you`; the count is additionally hard-scoped to
/// `you`'s permanents so a plain `ObjectFilter::creature()` already
/// means "creatures you control".
fn count_you_control(state: &GameState, you: PlayerId, filter: &ObjectFilter) -> u32 {
    state
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == you && filter.matches(o, state, you))
        .count() as u32
}

/// "if you control N or more [filter]" (metalcraft, threshold-style).
pub fn you_control_at_least(
    state: &GameState,
    you: PlayerId,
    filter: &ObjectFilter,
    n: u32,
) -> bool {
    count_you_control(state, you, filter) >= n
}

/// "if you control N or fewer [filter]".
pub fn you_control_at_most(
    state: &GameState,
    you: PlayerId,
    filter: &ObjectFilter,
    n: u32,
) -> bool {
    count_you_control(state, you, filter) <= n
}

/// "if you control a/an [filter]" (one or more).
pub fn you_control_a(state: &GameState, you: PlayerId, filter: &ObjectFilter) -> bool {
    count_you_control(state, you, filter) >= 1
}

/// "if you have N or more life".
pub fn life_at_least(state: &GameState, you: PlayerId, n: i32) -> bool {
    valid(state, you) && state.player(you).life >= n
}

/// "if you have N or less life".
pub fn life_at_most(state: &GameState, you: PlayerId, n: i32) -> bool {
    valid(state, you) && state.player(you).life <= n
}

/// "if you have N or more cards in hand".
pub fn hand_at_least(state: &GameState, you: PlayerId, n: u32) -> bool {
    crate::script::hand_size(state, you) >= n
}

/// "if you have no cards in hand" (hellbent).
pub fn hand_empty(state: &GameState, you: PlayerId) -> bool {
    crate::script::hand_size(state, you) == 0
}

/// "if there are N or more cards in your graveyard" (threshold/delirium
/// counts use a filtered graveyard — see [`crate::script::graveyard_matching`]).
pub fn graveyard_at_least(state: &GameState, you: PlayerId, n: u32) -> bool {
    crate::script::graveyard_size(state, you) >= n
}

/// "if [this permanent] has a [kind] counter on it" (≥1). Uses the
/// ability's `source`, which is why intervening-if fns receive it.
pub fn source_has_counter(state: &GameState, source: ObjectId, kind: CounterKind) -> bool {
    state
        .objects
        .get(source)
        .is_some_and(|o| o.count_counters(kind) > 0)
}

/// "if [this permanent] has N or more [kind] counters on it".
pub fn source_counters_at_least(
    state: &GameState,
    source: ObjectId,
    kind: CounterKind,
    n: u32,
) -> bool {
    state
        .objects
        .get(source)
        .is_some_and(|o| o.count_counters(kind) >= n)
}

/// "if no spells were cast last turn" — the werewolf front→back
/// transform trigger (Innistrad day/night precursor; CR 726.4 day→night).
pub fn no_spells_cast_last_turn(state: &GameState) -> bool {
    crate::script::spells_cast_last_turn_total(state) == 0
}

/// "if a player cast two or more spells last turn" — the werewolf
/// back→front transform trigger (CR 726.4 night→day).
pub fn a_player_cast_two_or_more_last_turn(state: &GameState) -> bool {
    crate::script::max_spells_by_a_player_last_turn(state) >= 2
}

/// CR 726 — "if it's day".
pub fn it_is_day(state: &GameState) -> bool {
    state.day_night == crate::turn::DayNight::Day
}

/// CR 726 — "if it's night".
pub fn it_is_night(state: &GameState) -> bool {
    state.day_night == crate::turn::DayNight::Night
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::{Characteristics, GameObject};
    use crate::types::{PtValue, TypeLine};

    fn put(s: &mut GameState, owner: PlayerId, chars: Characteristics) -> ObjectId {
        let id = s.allocate_object_id();
        let mut o = GameObject::new(id, owner, Zone::Battlefield, 0, chars);
        o.controller = owner;
        s.objects.insert(o);
        id
    }
    fn artifact() -> Characteristics {
        Characteristics { types: TypeLine::ARTIFACT.into(), ..Default::default() }
    }
    fn creature(p: i32, t: i32) -> Characteristics {
        Characteristics {
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
            ..Default::default()
        }
    }

    #[test]
    fn you_control_counts_only_your_permanents() {
        let mut s = GameState::new(2, 0);
        put(&mut s, 0, artifact());
        put(&mut s, 0, artifact());
        put(&mut s, 1, artifact()); // opponent's — must not count
        let f = ObjectFilter { types: Some(TypeLine::ARTIFACT.into()), ..Default::default() };
        assert!(you_control_at_least(&s, 0, &f, 2));
        assert!(!you_control_at_least(&s, 0, &f, 3));
        assert!(you_control_at_most(&s, 0, &f, 2));
        assert!(you_control_a(&s, 0, &f));
        assert!(!you_control_a(&s, 1, &ObjectFilter::creature()));
    }

    #[test]
    fn life_and_hand_predicates_are_total() {
        let mut s = GameState::new(2, 0);
        // default starting life is 20.
        assert!(life_at_least(&s, 0, 20));
        assert!(life_at_most(&s, 0, 20));
        assert!(!life_at_most(&s, 0, 19));
        assert!(hand_empty(&s, 0));
        assert!(!hand_at_least(&s, 0, 1));
        // invalid player → neutral (false), no panic.
        assert!(!life_at_least(&s, 99, 1));
        let _ = put(&mut s, 0, creature(1, 1));
    }

    #[test]
    fn last_turn_spell_predicates_and_day_night_transition() {
        use crate::events::GameEvent;
        use crate::targets::TargetSelection;
        use crate::turn::DayNight;
        let mut s = GameState::new(2, 0);
        let cast = |c: PlayerId| GameEvent::SpellCast {
            object_id: 1, card_id: 1, controller: c, targets: TargetSelection::new(),
        };
        // Last turn: player 0 cast two spells.
        s.prev_turn_event_log_start = s.event_log.len();
        s.event_log.push(cast(0));
        s.event_log.push(cast(0));
        s.turn_event_log_start = s.event_log.len();
        assert!(!no_spells_cast_last_turn(&s));
        assert!(a_player_cast_two_or_more_last_turn(&s));
        // Night + a player cast 2+ last turn → becomes day (CR 726.4).
        s.day_night = DayNight::Night;
        s.apply_day_night_transition();
        assert_eq!(s.day_night, DayNight::Day);
        assert!(it_is_day(&s) && !it_is_night(&s));

        // A turn with no spells (empty last-turn slice).
        s.prev_turn_event_log_start = s.event_log.len();
        s.turn_event_log_start = s.event_log.len();
        assert!(no_spells_cast_last_turn(&s));
        assert!(!a_player_cast_two_or_more_last_turn(&s));
        // Day + no spells last turn → becomes night.
        s.day_night = DayNight::Day;
        s.apply_day_night_transition();
        assert_eq!(s.day_night, DayNight::Night);

        // Neither never transitions.
        s.day_night = DayNight::Neither;
        s.apply_day_night_transition();
        assert_eq!(s.day_night, DayNight::Neither);
    }

    #[test]
    fn source_counter_predicates_read_the_source() {
        let mut s = GameState::new(2, 0);
        let c = put(&mut s, 0, creature(1, 1));
        assert!(!source_has_counter(&s, c, CounterKind::PlusOnePlusOne));
        s.objects.get_mut(c).unwrap().add_counters(CounterKind::PlusOnePlusOne, 2);
        assert!(source_has_counter(&s, c, CounterKind::PlusOnePlusOne));
        assert!(source_counters_at_least(&s, c, CounterKind::PlusOnePlusOne, 2));
        assert!(!source_counters_at_least(&s, c, CounterKind::PlusOnePlusOne, 3));
        // missing object → false, no panic.
        assert!(!source_has_counter(&s, 99999, CounterKind::PlusOnePlusOne));
    }
}
