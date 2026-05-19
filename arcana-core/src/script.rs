//! Card-scripting prelude (engine-gap Tier 1).
//!
//! A small, **total, panic-free, read-only** query surface a
//! generated card resolver may call to compute a *resolution-time*
//! amount (damage = a creature's power, draw = creatures you control,
//! …) or an id list (every Goblin on the battlefield, for a filtered
//! board wipe), then feed that plain value into an ordinary
//! literal-amount [`crate::effects::Effect`].
//!
//! Design contract — every function here:
//! * takes `&GameState` (never `&mut`) plus plain ids/filters,
//! * returns a plain value (`i32`/`u32`/`Vec<ObjectId>`), never
//!   `Option`/`Result`/an iterator, so a generated resolver stays
//!   trivial to compile (Layer 1) and to read,
//! * never panics — an out-of-range player or missing object yields
//!   the neutral value (`0` / empty), mirroring the defensive
//!   posture of [`crate::effects::Effect::execute`].
//!
//! This is the curated alternative to letting generated resolvers
//! reach into `GameState` internals freely (which the cardgen
//! API-discipline rule forbids precisely because ad-hoc state access
//! is the dominant Layer-1 break). The prompt exposes exactly these
//! helpers and a reference card; nothing else.

use crate::objects::ObjectId;
use crate::state::GameState;
use crate::targets::ObjectFilter;
use crate::types::PlayerId;
use crate::zones::Zone;

/// `true` iff `p` indexes a real player (guards the panicking
/// [`GameState::player`]).
fn valid(state: &GameState, p: PlayerId) -> bool {
    p < state.num_players()
}

/// Battlefield permanents matching `filter`. `you` is the resolving
/// controller — it disambiguates `ControllerConstraint::You`/
/// `Opponent` inside the filter (pass `entry.controller`). Ids are
/// returned in the arena's stable battlefield order so a
/// [`crate::effects::Effect::ForEach`] over them is deterministic.
pub fn ids_matching(
    state: &GameState,
    filter: &ObjectFilter,
    you: PlayerId,
) -> Vec<ObjectId> {
    state
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| filter.matches(o, state, you))
        .map(|o| o.id)
        .collect()
}

/// Count of battlefield permanents matching `filter` (see
/// [`ids_matching`]). The canonical "X = number of <things>"
/// resolution-time amount.
pub fn count_matching(
    state: &GameState,
    filter: &ObjectFilter,
    you: PlayerId,
) -> u32 {
    state
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| filter.matches(o, state, you))
        .count() as u32
}

/// Cards in `player`'s graveyard matching `filter`. `0` for an
/// invalid player.
pub fn graveyard_matching(
    state: &GameState,
    filter: &ObjectFilter,
    player: PlayerId,
    you: PlayerId,
) -> u32 {
    if !valid(state, player) {
        return 0;
    }
    state
        .objects
        .objects_in_zone(Zone::Graveyard(player))
        .filter(|o| filter.matches(o, state, you))
        .count() as u32
}

/// `id`'s current power after the layer system, or `0` if `id` is
/// gone / not a creature. Use for "deals damage equal to its power".
pub fn power_of(state: &GameState, id: ObjectId) -> i32 {
    state.computed_power(id).unwrap_or(0)
}

/// `id`'s current toughness after the layer system, or `0`.
pub fn toughness_of(state: &GameState, id: ObjectId) -> i32 {
    state.computed_toughness(id).unwrap_or(0)
}

/// Number of cards in `player`'s hand (`0` for an invalid player).
pub fn hand_size(state: &GameState, player: PlayerId) -> u32 {
    if !valid(state, player) {
        return 0;
    }
    state.objects.count_in_zone(Zone::Hand(player)) as u32
}

/// Number of cards in `player`'s graveyard.
pub fn graveyard_size(state: &GameState, player: PlayerId) -> u32 {
    if !valid(state, player) {
        return 0;
    }
    state.objects.count_in_zone(Zone::Graveyard(player)) as u32
}

/// Number of cards in `player`'s library.
pub fn library_size(state: &GameState, player: PlayerId) -> u32 {
    if !valid(state, player) {
        return 0;
    }
    state.player(player).library_top_to_bottom.len() as u32
}

/// `player`'s life total (`0` for an invalid player).
pub fn life(state: &GameState, player: PlayerId) -> i32 {
    if !valid(state, player) {
        return 0;
    }
    state.player(player).life
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::objects::{Characteristics, GameObject};
    use crate::types::{ColorSet, PtValue, TypeLine};

    fn creature_chars(p: i32, t: i32) -> Characteristics {
        Characteristics {
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            power: Some(PtValue::Fixed(p)),
            toughness: Some(PtValue::Fixed(t)),
            ..Default::default()
        }
    }

    fn put(state: &mut GameState, zone: Zone, owner: PlayerId,
           chars: Characteristics) -> ObjectId {
        let id = state.allocate_object_id();
        let mut o = GameObject::new(id, owner, zone, 0, chars);
        o.controller = owner;
        state.objects.insert(o);
        id
    }

    #[test]
    fn count_and_ids_matching_filter_the_battlefield() {
        let mut s = GameState::new(2, 0);
        put(&mut s, Zone::Battlefield, 0, creature_chars(2, 2));
        put(&mut s, Zone::Battlefield, 0, creature_chars(4, 4));
        // A non-creature permanent: should not match a creature filter.
        let land = Characteristics {
            types: TypeLine::LAND.into(), ..Default::default()
        };
        put(&mut s, Zone::Battlefield, 1, land);
        let f = ObjectFilter::creature();
        assert_eq!(count_matching(&s, &f, 0), 2);
        assert_eq!(ids_matching(&s, &f, 0).len(), 2);
        // "Creatures you control" — controller constraint resolved
        // against `you`.
        let yours = ObjectFilter::creature()
            .controlled_by(crate::targets::ControllerConstraint::You);
        assert_eq!(count_matching(&s, &yours, 0), 2);
        assert_eq!(count_matching(&s, &yours, 1), 0);
    }

    #[test]
    fn power_toughness_of_missing_is_zero() {
        let mut s = GameState::new(2, 0);
        let c = put(&mut s, Zone::Battlefield, 0, creature_chars(3, 5));
        assert_eq!(power_of(&s, c), 3);
        assert_eq!(toughness_of(&s, c), 5);
        assert_eq!(power_of(&s, 9999), 0);
        assert_eq!(toughness_of(&s, 9999), 0);
    }

    #[test]
    fn zone_sizes_and_life_are_total() {
        let mut s = GameState::new(2, 0);
        put(&mut s, Zone::Hand(0), 0, creature_chars(1, 1));
        put(&mut s, Zone::Hand(0), 0, creature_chars(1, 1));
        put(&mut s, Zone::Graveyard(0), 0, creature_chars(1, 1));
        assert_eq!(hand_size(&s, 0), 2);
        assert_eq!(graveyard_size(&s, 0), 1);
        assert_eq!(library_size(&s, 0), 0);
        assert_eq!(life(&s, 0), s.player(0).life);
        // Out-of-range player: neutral, never panics.
        assert_eq!(hand_size(&s, 99), 0);
        assert_eq!(life(&s, 99), 0);
        assert_eq!(library_size(&s, 99), 0);
    }

    #[test]
    fn graveyard_matching_respects_filter() {
        let mut s = GameState::new(2, 0);
        put(&mut s, Zone::Graveyard(0), 0, creature_chars(2, 2));
        let inst = Characteristics {
            types: TypeLine::INSTANT.into(), ..Default::default()
        };
        put(&mut s, Zone::Graveyard(0), 0, inst);
        assert_eq!(
            graveyard_matching(&s, &ObjectFilter::creature(), 0, 0), 1);
        assert_eq!(graveyard_size(&s, 0), 2);
    }
}
