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
use crate::registry::CardRegistry;
use crate::state::GameState;
use crate::targets::ObjectFilter;
use crate::types::PlayerId;
use crate::zones::Zone;

/// An [`ObjectFilter`] for creatures of a given subtype name
/// (`"Goblin"`, `"Zombie"`, …), resolving the interned symbol via
/// `reg` so a generated resolver never touches the interner. If the
/// subtype was never interned (no card of that type exists in the
/// catalog) the returned filter **matches nothing** — total and
/// safe. Chain the ordinary [`ObjectFilter`] builders for further
/// refinement, e.g.
/// `script::subtype_filter(reg, "Goblin").controlled_by(You)`.
pub fn subtype_filter(reg: &CardRegistry, subtype: &str) -> ObjectFilter {
    match reg.interner().lookup(subtype) {
        Some(sym) => ObjectFilter::creature().with_subtype_sym(sym),
        // Never interned ⇒ a filter that matches no object.
        None => ObjectFilter {
            custom: Some(|_, _| false),
            ..ObjectFilter::default()
        },
    }
}

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

/// Controller of object `id` ("that creature's controller", "its
/// owner" — most cards conflate the two for non-stolen permanents).
/// Falls back to `default` (typically `entry.controller`) when the
/// object is gone, so generated code stays total. Use for
/// `CreateToken {{ controller: script::target_controller(state, id,
/// entry.controller) }}` when the oracle text says "target's
/// controller gets a token" / "create a token under that player's
/// control".
pub fn target_controller(
    state: &GameState,
    id: ObjectId,
    default: PlayerId,
) -> PlayerId {
    state.objects.get(id).map(|o| o.controller).unwrap_or(default)
}

/// Every player id in turn order. For "each player draws/discards/
/// loses life": map this into one inner [`crate::effects::Effect`]
/// per player and wrap in `Effect::Sequence`.
pub fn all_players(state: &GameState) -> Vec<PlayerId> {
    (0..state.num_players()).collect()
}

/// Every player except `you` (the resolving controller). For "each
/// opponent …".
pub fn opponents(state: &GameState, you: PlayerId) -> Vec<PlayerId> {
    (0..state.num_players()).filter(|&p| p != you).collect()
}

/// `player`'s life total (`0` for an invalid player).
pub fn life(state: &GameState, player: PlayerId) -> i32 {
    if !valid(state, player) {
        return 0;
    }
    state.player(player).life
}

// =============================================================================
// Per-turn typed-event counters (Phase A #6)
// =============================================================================
//
// Each helper scans `state.event_log[state.turn_event_log_start..]` —
// the slice for the current turn — and returns the count matching a
// predicate. The turn-start cursor is bumped to `event_log.len()` on
// every TurnBegins emission, so every helper is O(events-this-turn).
//
// "this turn" semantics mean only the *active* turn — the slice resets
// when a new turn begins. Use these for cards that read
// "[creatures of type X] died this turn", "spells you've cast this
// turn", "[player] drew/discarded a card this turn", and similar.

/// Slice of `state.event_log` covering only the current turn.
fn this_turn_events(state: &GameState) -> &[crate::events::GameEvent] {
    let start = state.turn_event_log_start.min(state.event_log.len());
    &state.event_log[start..]
}

/// Number of creatures with `subtype` (interned via `reg`) that died
/// this turn — used for Silent-Chant Zubera class ("you gain 2 life
/// for each Zubera that died this turn"). Reads
/// [`crate::events::GameEvent::Dies`] for objects whose LKI carries
/// `subtype`. If `subtype` was never interned the count is 0.
pub fn creatures_of_subtype_died_this_turn(
    state: &GameState,
    reg: &CardRegistry,
    subtype: &str,
) -> u32 {
    let Some(sym) = reg.interner().lookup(subtype) else { return 0; };
    let mut n = 0u32;
    for ev in this_turn_events(state) {
        if let crate::events::GameEvent::Dies { object_id } = ev {
            // Live arena first, then LKI (the typical case — the
            // creature just left the battlefield).
            let chars = state.objects.get(*object_id)
                .map(|o| &o.characteristics)
                .or_else(|| state.lki.get(object_id).map(|o| &o.characteristics));
            if let Some(c) = chars {
                if c.subtypes.contains(sym) { n += 1; }
            }
        }
    }
    n
}

/// Number of spells cast this turn matching `filter`. `you` resolves
/// `ControllerConstraint::You` / `Opponent` inside the filter. For
/// "instant or sorcery spells you've cast this turn" pass
/// `ObjectFilter::new().with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)).controlled_by(ControllerConstraint::You)`.
pub fn spells_cast_this_turn(
    state: &GameState,
    filter: &ObjectFilter,
    you: PlayerId,
) -> u32 {
    let mut n = 0u32;
    for ev in this_turn_events(state) {
        if let crate::events::GameEvent::SpellCast { object_id, .. } = ev {
            // Look the spell up via arena or LKI to inspect chars.
            let obj = state.objects.get(*object_id)
                .or_else(|| state.lki.get(object_id));
            if let Some(o) = obj {
                if filter.matches(o, state, you) { n += 1; }
            }
        }
    }
    n
}

/// Number of cards `player` has drawn this turn. Counts
/// [`crate::events::GameEvent::CardDrawn`] events filtered to
/// `player`. (`0` for an invalid player.)
pub fn cards_drawn_this_turn(
    state: &GameState,
    player: PlayerId,
) -> u32 {
    if !valid(state, player) { return 0; }
    this_turn_events(state).iter()
        .filter(|ev| matches!(ev,
            crate::events::GameEvent::DrawCard { player: p, .. } if *p == player))
        .count() as u32
}

/// Number of cards `player` has discarded this turn — for
/// "discarded this turn" / Madness-adjacent / "for each card you've
/// discarded this turn" scaling.
pub fn cards_discarded_this_turn(
    state: &GameState,
    player: PlayerId,
) -> u32 {
    if !valid(state, player) { return 0; }
    this_turn_events(state).iter()
        .filter(|ev| matches!(ev,
            crate::events::GameEvent::Discarded { player: p, .. } if *p == player))
        .count() as u32
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
    fn subtype_filter_resolves_or_matches_nothing() {
        use crate::registry::CardRegistry;
        let mut reg = CardRegistry::new();
        let goblin = reg.interner_mut().intern("Goblin");
        let mut s = GameState::new(2, 0);
        let mut gob = creature_chars(1, 1);
        gob.subtypes.0.insert(goblin);
        put(&mut s, Zone::Battlefield, 0, gob);
        put(&mut s, Zone::Battlefield, 0, creature_chars(2, 2)); // no subtype
        let f = subtype_filter(&reg, "Goblin");
        assert_eq!(count_matching(&s, &f, 0), 1);
        // Never-interned subtype ⇒ matches nothing, no panic.
        let none = subtype_filter(&reg, "Eldrazi");
        assert_eq!(count_matching(&s, &none, 0), 0);
    }

    #[test]
    fn numeric_builders_narrow_the_set() {
        let mut s = GameState::new(2, 0);
        let mut big = creature_chars(5, 5);
        big.mana_cost = Some(crate::mana::ManaCost::parse("{4}{G}").unwrap());
        put(&mut s, Zone::Battlefield, 0, big);
        let mut small = creature_chars(1, 1);
        small.mana_cost = Some(crate::mana::ManaCost::parse("{G}").unwrap());
        put(&mut s, Zone::Battlefield, 0, small);
        assert_eq!(
            count_matching(&s, &ObjectFilter::creature().with_max_cmc(2), 0), 1);
        assert_eq!(
            count_matching(&s, &ObjectFilter::creature().with_min_power(3), 0), 1);
    }

    #[test]
    fn color_exclusion_and_tap_filters() {
        let mut s = GameState::new(2, 0);
        // green creature (creature_chars uses ColorSet::green)
        let g = put(&mut s, Zone::Battlefield, 0, creature_chars(2, 2));
        let mut blk = creature_chars(2, 2);
        blk.colors = crate::types::ColorSet::black();
        put(&mut s, Zone::Battlefield, 0, blk);
        // "noncolorless... nonblack creature" → only the green one
        let nonblack = ObjectFilter::creature()
            .without_colors(crate::types::ColorSet::black());
        assert_eq!(count_matching(&s, &nonblack, 0), 1);
        // tap the green one; "tapped creature" matches just it
        s.objects.get_mut(g).unwrap().tap();
        assert_eq!(
            count_matching(&s, &ObjectFilter::creature().tapped_only(), 0), 1);
        assert_eq!(
            count_matching(&s, &ObjectFilter::creature().untapped_only(), 0), 1);
    }

    #[test]
    fn target_controller_reads_object_else_default() {
        let mut s = GameState::new(2, 0);
        let c = put(&mut s, Zone::Battlefield, 1, creature_chars(1, 1));
        assert_eq!(target_controller(&s, c, 0), 1);
        assert_eq!(target_controller(&s, 9999, 0), 0,
            "missing object → falls back to default");
    }

    #[test]
    fn player_lists_for_each_player_effects() {
        let s = GameState::new(3, 0);
        assert_eq!(all_players(&s), vec![0, 1, 2]);
        assert_eq!(opponents(&s, 1), vec![0, 2]);
        assert_eq!(opponents(&s, 0), vec![1, 2]);
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

    // --- per-turn counters (Phase A #6) -----------------------------

    #[test]
    fn this_turn_slice_respects_cursor() {
        let mut s = GameState::new(2, 0);
        // Pre-turn event (e.g. from setup).
        s.emit(crate::events::GameEvent::DrawCard {
            player: 0, object_id: 1 });
        // Bump the cursor as if a new turn began.
        s.turn_event_log_start = s.event_log.len();
        s.emit(crate::events::GameEvent::DrawCard {
            player: 0, object_id: 2 });
        s.emit(crate::events::GameEvent::DrawCard {
            player: 0, object_id: 3 });
        s.emit(crate::events::GameEvent::DrawCard {
            player: 1, object_id: 4 });
        assert_eq!(cards_drawn_this_turn(&s, 0), 2,
            "pre-turn draw is excluded; both this-turn draws by p0 count");
        assert_eq!(cards_drawn_this_turn(&s, 1), 1);
    }

    #[test]
    fn cards_discarded_this_turn_counts_per_player() {
        let mut s = GameState::new(2, 0);
        s.turn_event_log_start = 0;
        s.emit(crate::events::GameEvent::Discarded {
            player: 0, object_id: 1 });
        s.emit(crate::events::GameEvent::Discarded {
            player: 0, object_id: 2 });
        assert_eq!(cards_discarded_this_turn(&s, 0), 2);
        assert_eq!(cards_discarded_this_turn(&s, 1), 0);
    }

    #[test]
    fn creatures_of_subtype_died_this_turn_via_lki() {
        // The dying creature has typically left the battlefield by the
        // time the resolver runs — we must read characteristics from
        // LKI. Set up two dies events: one with the subtype, one without.
        let mut s = GameState::new(2, 0);
        let mut reg = CardRegistry::new();
        let zubera = reg.interner_mut().intern("Zubera");
        let _ = reg.interner_mut().intern("Goblin"); // ensure interned

        let mut subtypes = crate::types::SubtypeSet::default();
        subtypes.0.insert(zubera);
        let zubera_chars = Characteristics {
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            ..Default::default()
        };
        let goblin_chars = creature_chars(2, 2); // no Zubera subtype

        // Materialize the creatures, then move them into LKI to mimic
        // post-death state.
        let z1 = put(&mut s, Zone::Battlefield, 0, zubera_chars.clone());
        let z2 = put(&mut s, Zone::Battlefield, 0, zubera_chars);
        let g  = put(&mut s, Zone::Battlefield, 0, goblin_chars);
        // Snapshot into LKI before they 'die'.
        for id in [z1, z2, g] {
            let obj = s.objects.get(id).unwrap().clone();
            s.lki.insert(id, obj);
        }
        s.turn_event_log_start = 0;
        s.emit(crate::events::GameEvent::Dies { object_id: z1 });
        s.emit(crate::events::GameEvent::Dies { object_id: z2 });
        s.emit(crate::events::GameEvent::Dies { object_id: g });

        assert_eq!(creatures_of_subtype_died_this_turn(&s, &reg, "Zubera"), 2);
        assert_eq!(creatures_of_subtype_died_this_turn(&s, &reg, "Goblin"), 0,
            "the Goblin object had no subtype set in this helper test");
        // Unknown subtype (never interned) -> 0.
        assert_eq!(creatures_of_subtype_died_this_turn(&s, &reg, "Sliver"), 0);
    }
}
