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

/// An [`ObjectFilter`] for PERMANENTS of a given subtype name
/// (`"Goblin"`, `"Zombie"`, `"Mountain"`, `"Equipment"`, …), resolving
/// the interned symbol via `reg` so a generated resolver never touches
/// the interner. If the subtype was never interned (no card of that
/// type exists in the catalog) the returned filter **matches nothing**
/// — total and safe. Chain the ordinary [`ObjectFilter`] builders for
/// further refinement, e.g.
/// `script::subtype_filter(reg, "Goblin").controlled_by(You)`.
///
/// Permanent-scoped, NOT creature-scoped: a subtype belongs to whatever
/// permanent bears it. Creature subtypes (Goblin/Elf) are overwhelmingly
/// on creatures so this is equivalent there, but LAND subtypes
/// (Mountain/Forest/…) live on lands — a creature-scoped filter counted
/// zero of them, silently breaking every "Mountains you control" /
/// "destroy all Swamps" card (caught by the behavioral probe). Add
/// `.creature()`-style refinements if a card truly means "creatures of
/// this subtype".
pub fn subtype_filter(reg: &CardRegistry, subtype: &str) -> ObjectFilter {
    // Commodity-token "subtypes" (Treasure/Clue/Food/…) are token-only
    // in real Magic and are minted engine-side without an interner, so
    // they never carry a subtype symbol — match them by the kind marker
    // instead. One chokepoint fixes every "sacrifice a Treasure" /
    // "for each Food you control" card.
    if let Some(kind) = commodity_kind_by_name(subtype) {
        return ObjectFilter::permanent().commodity_kind(kind);
    }
    match reg.interner().lookup(subtype) {
        Some(sym) => ObjectFilter::permanent().with_subtype_sym(sym),
        // Never interned ⇒ a filter that matches no object.
        None => ObjectFilter {
            custom: Some(|_, _| false),
            ..ObjectFilter::default()
        },
    }
}

/// Map a commodity-token subtype name to its [`CommodityToken`] kind, or
/// `None` for any ordinary subtype. Mirrors
/// [`crate::effects::CommodityToken::display_name`].
pub fn commodity_kind_by_name(subtype: &str) -> Option<crate::effects::CommodityToken> {
    use crate::effects::CommodityToken::*;
    Some(match subtype {
        "Treasure" => Treasure,
        "Clue" => Clue,
        "Food" => Food,
        "Powerstone" => Powerstone,
        "Incubator" => Incubator,
        "Blood" => Blood,
        "Map" => Map,
        _ => return None,
    })
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

/// CR 702.27 — Domain: the number of basic land types (Plains, Island,
/// Swamp, Mountain, Forest) among lands `you` control. Each type counts
/// once no matter how many lands bear it, and a single land with several
/// basic types (a dual land, or a land made all-basic-types by Urborg /
/// Prismatic Omen) contributes each of its types. Range `0..=5`. The
/// canonical "X = your domain" resolution amount and the
/// [`crate::registry::ActivationCostReductionFn`] count for "{1} less per
/// basic land type among lands you control".
///
/// Reads the lands' current subtypes (so layer-granted basic land types
/// count); resolves the five basic-type symbols via `reg`, returning `0`
/// for any never interned. Total and panic-free.
pub fn domain(state: &GameState, you: PlayerId, reg: &CardRegistry) -> u32 {
    if !valid(state, you) {
        return 0;
    }
    const BASICS: [&str; 5] = ["Plains", "Island", "Swamp", "Mountain", "Forest"];
    let syms: Vec<_> = BASICS
        .iter()
        .filter_map(|b| reg.interner().lookup(b))
        .collect();
    syms.iter()
        .filter(|&&sym| {
            state
                .objects
                .objects_in_zone(Zone::Battlefield)
                .any(|o| {
                    o.controller == you
                        && o.is_land()
                        && o.characteristics.subtypes.contains(sym)
                })
        })
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

/// CR 700.5 — Devotion to `colors`: the number of mana symbols of
/// those colors among the mana costs of permanents `you` control. Each
/// qualifying symbol counts once (a hybrid `{W/U}` symbol counts for
/// devotion to white, to blue, and to white-or-blue alike; Phyrexian
/// `{W/P}` counts as a white symbol; generic/`{C}`/`{X}` never count).
/// Pass a single color (`ColorSet::white()`) or a union
/// (`ColorSet::white() | ColorSet::blue()` for devotion to white and
/// blue). The canonical "X = your devotion to ~" resolution amount.
pub fn devotion(state: &GameState, you: PlayerId, colors: crate::types::ColorSet) -> u32 {
    if colors.0 == 0 || !valid(state, you) {
        return 0;
    }
    state
        .objects
        .objects_in_zone(Zone::Battlefield)
        .filter(|o| o.controller == you)
        .map(|o| {
            o.characteristics.mana_cost.as_ref().map_or(0, |mc| {
                mc.components
                    .iter()
                    .filter(|c| (c.colors().0 & colors.0) != 0)
                    .count() as u32
            })
        })
        .sum()
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

/// Number of spells matching `filter` that `caster` (specifically) has
/// cast this turn — the per-CASTER sibling of [`spells_cast_this_turn`]
/// (which counts across all players). For "each player can't cast more
/// than one spell each turn" (Rule of Law / Arcane Laboratory) and its
/// filtered kin (Deafening Silence's noncreature, Ethersworn Canonist's
/// artifact). `caster` resolves the filter's controller constraint too.
pub fn spells_cast_this_turn_by(
    state: &GameState,
    filter: &ObjectFilter,
    caster: PlayerId,
) -> u32 {
    let mut n = 0u32;
    for ev in this_turn_events(state) {
        if let crate::events::GameEvent::SpellCast { object_id, controller, .. } = ev {
            if *controller != caster { continue; }
            let obj = state.objects.get(*object_id)
                .or_else(|| state.lki.get(object_id));
            if let Some(o) = obj {
                if filter.matches(o, state, caster) { n += 1; }
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

/// Number of `kind` counters on `source` (the ability's own source
/// object) — the resolution-time amount for "for each [counter] on
/// this" / "X is the number of [counter] counters on ~" payoffs
/// (counter-accumulation enchantments: Assemble the Legion's muster →
/// Soldiers, Descent into Avernus' descent → Treasures + damage, Mind
/// Unbound's lore → draw). `0` for an invalid/absent source.
///
/// IMPORTANT (same-resolution adds): when the SAME ability both ADDS a
/// counter and reads "for each counter on this" (the count includes the
/// just-added one per the card's wording), this accessor reads the count
/// BEFORE the ability's own `Effect::AddCounters` applies — so add the
/// printed increment: `source_counter_count(state, src, kind) + <added>`.
pub fn source_counter_count(
    state: &GameState,
    source: ObjectId,
    kind: crate::types::CounterKind,
) -> u32 {
    state.objects.get(source)
        .map(|o| o.count_counters(kind))
        .unwrap_or(0)
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

/// Creatures currently blocking `attacker` (CR 509) — "each creature
/// blocking it" sweeps (Battle-Scarred Goblin, Electryte, Gang of
/// Elk). Empty outside combat or when unblocked. Declared order.
pub fn blockers_of(state: &GameState, attacker: ObjectId) -> Vec<ObjectId> {
    state.combat.as_ref()
        .and_then(|c| c.attacker(attacker))
        .map(|a| a.blocked_by.clone())
        .unwrap_or_default()
}

/// Attackers that `blocker` is currently blocking — the inverse
/// pairing ("each creature it's blocking"). Empty outside combat.
pub fn attackers_blocked_by(state: &GameState, blocker: ObjectId) -> Vec<ObjectId> {
    state.combat.as_ref()
        .map(|c| c.blockers.iter()
            .filter(|b| b.object_id == blocker)
            .map(|b| b.blocking)
            .collect())
        .unwrap_or_default()
}

/// Did `id` attack this turn? Scans [`crate::events::GameEvent::CreatureAttacks`]
/// for the live turn. The Boast gate ("activate only if this creature
/// attacked this turn") — `id` is stable while the creature stays on
/// the battlefield, which is the only zone Boast activates from.
pub fn creature_attacked_this_turn(state: &GameState, id: ObjectId) -> bool {
    this_turn_events(state).iter().any(|ev| matches!(ev,
        crate::events::GameEvent::CreatureAttacks { attacker, .. } if *attacker == id))
}

/// Did `player` attack with any creature this turn? The attacker's
/// controller is read from the live arena or LKI (controller at attack
/// time ≈ controller now; a mid-combat control change after declaring
/// is not tracked — documented approximation).
pub fn player_attacked_this_turn(state: &GameState, player: PlayerId) -> bool {
    this_turn_events(state).iter().any(|ev| {
        if let crate::events::GameEvent::CreatureAttacks { attacker, .. } = ev {
            state.objects.get(*attacker)
                .or_else(|| state.lki.get(attacker))
                .is_some_and(|o| o.controller == player)
        } else {
            false
        }
    })
}

/// Creatures (and planeswalkers — both emit `Dies`) that died this
/// turn, ANY controller. Morbid ("if a creature died this turn") and
/// "for each creature that died this turn" scaling.
pub fn creatures_died_this_turn(state: &GameState) -> u32 {
    this_turn_events(state).iter()
        .filter(|ev| matches!(ev, crate::events::GameEvent::Dies { .. }))
        .count() as u32
}

/// Total life `player` has lost this turn. Sums
/// [`crate::events::GameEvent::LifeLost`], which the engine emits for
/// BOTH effect-driven loss and damage to the player (combat or spell),
/// per CR 120.3.
pub fn life_lost_this_turn(state: &GameState, player: PlayerId) -> u32 {
    if !valid(state, player) { return 0; }
    this_turn_events(state).iter()
        .filter_map(|ev| match ev {
            crate::events::GameEvent::LifeLost { player: p, amount } if *p == player =>
                Some(*amount),
            _ => None,
        })
        .sum()
}

/// Total life `player` has gained this turn.
pub fn life_gained_this_turn(state: &GameState, player: PlayerId) -> u32 {
    if !valid(state, player) { return 0; }
    this_turn_events(state).iter()
        .filter_map(|ev| match ev {
            crate::events::GameEvent::LifeGained { player: p, amount } if *p == player =>
                Some(*amount),
            _ => None,
        })
        .sum()
}

/// Did `id` enter the battlefield this turn? Per-object timestamp
/// check ("if ~ entered the battlefield this turn").
pub fn entered_battlefield_this_turn(state: &GameState, id: ObjectId) -> bool {
    this_turn_events(state).iter().any(|ev| matches!(ev,
        crate::events::GameEvent::EntersBattlefield { object_id, .. } if *object_id == id))
}

/// Permanents matching `filter` that entered the battlefield this turn.
/// `you` resolves the filter's controller constraints — "a creature
/// entered the battlefield under your control this turn" =
/// `ObjectFilter::creature().controlled_by(ControllerConstraint::You)`.
/// Entrants are looked up via arena-then-LKI (an entrant that already
/// left still counts — it did enter).
pub fn entered_this_turn_matching(
    state: &GameState,
    filter: &ObjectFilter,
    you: PlayerId,
) -> u32 {
    let mut n = 0u32;
    for ev in this_turn_events(state) {
        if let crate::events::GameEvent::EntersBattlefield { object_id, .. } = ev {
            let obj = state.objects.get(*object_id)
                .or_else(|| state.lki.get(object_id));
            if let Some(o) = obj {
                if filter.matches(o, state, you) { n += 1; }
            }
        }
    }
    n
}

/// Events of the turn that JUST ended — the slice between last turn's
/// start marker and this turn's. Empty on turn one. Powers the
/// werewolf transform condition and the CR 726.4 day/night flip.
fn last_turn_events(state: &GameState) -> &[crate::events::GameEvent] {
    let start = state.prev_turn_event_log_start.min(state.event_log.len());
    let end = state.turn_event_log_start.min(state.event_log.len());
    if start > end { return &[]; }
    &state.event_log[start..end]
}

/// Total spells cast last turn by ALL players. `0` is the werewolf
/// front→night transform trigger ("if no spells were cast last turn").
pub fn spells_cast_last_turn_total(state: &GameState) -> u32 {
    last_turn_events(state).iter()
        .filter(|ev| matches!(ev, crate::events::GameEvent::SpellCast { .. }))
        .count() as u32
}

/// The most spells cast by any single player last turn. `>= 2` is the
/// werewolf back→day transform trigger ("if a player cast two or more
/// spells last turn").
pub fn max_spells_by_a_player_last_turn(state: &GameState) -> u32 {
    let mut counts: crate::collections::HashMap<PlayerId, u32> = Default::default();
    let mut max = 0u32;
    for ev in last_turn_events(state) {
        if let crate::events::GameEvent::SpellCast { controller, .. } = ev {
            let c = counts.entry(*controller).or_insert(0);
            *c += 1;
            max = max.max(*c);
        }
    }
    max
}

/// The greatest mana value among battlefield permanents matching `filter`
/// (0 if none). For "the greatest mana value among permanents/artifacts" or
/// "X is the highest mana value …" amounts.
pub fn max_cmc_of(state: &GameState, filter: &ObjectFilter, you: PlayerId) -> u32 {
    ids_matching(state, filter, you).into_iter()
        .filter_map(|id| state.objects.get(id))
        .map(|o| o.characteristics.mana_value())
        .max()
        .unwrap_or(0)
}

/// The greatest power among battlefield permanents matching `filter` (0 if
/// none; negatives clamp via the caller's `.max(0) as u32`). For "equal to the
/// greatest power among …" amounts.
pub fn max_power_of(state: &GameState, filter: &ObjectFilter, you: PlayerId) -> i32 {
    ids_matching(state, filter, you).into_iter()
        .map(|id| power_of(state, id))
        .max()
        .unwrap_or(0)
}

/// The interned name handle of `id` (or `None` if it's gone). Compare two
/// handles for "a creature with the same name" — name equality is symbol
/// equality within one registry. No registry param needed; the handle lives on
/// the object's characteristics.
pub fn name_of(state: &GameState, id: ObjectId) -> Option<crate::types::SmallString> {
    state.objects.get(id).map(|o| o.characteristics.name)
}

/// Number of DISTINCT card types among cards in `player`'s graveyard (delirium —
/// "four or more card types"; "draw for each card type in your graveyard").
/// Counts the eight card-type categories present at least once.
pub fn distinct_card_types_in_graveyard(state: &GameState, player: PlayerId) -> u32 {
    if !valid(state, player) { return 0; }
    let g: Vec<_> = state.objects.objects_in_zone(Zone::Graveyard(player)).collect();
    let mut n = 0u32;
    if g.iter().any(|o| o.characteristics.types.is_artifact()) { n += 1; }
    if g.iter().any(|o| o.characteristics.types.is_battle()) { n += 1; }
    if g.iter().any(|o| o.characteristics.types.is_creature()) { n += 1; }
    if g.iter().any(|o| o.characteristics.types.is_enchantment()) { n += 1; }
    if g.iter().any(|o| o.characteristics.types.is_instant()) { n += 1; }
    if g.iter().any(|o| o.characteristics.types.is_land()) { n += 1; }
    if g.iter().any(|o| o.characteristics.types.is_planeswalker()) { n += 1; }
    if g.iter().any(|o| o.characteristics.types.is_sorcery()) { n += 1; }
    n
}

/// Number of DISTINCT mana values among battlefield permanents matching `filter`
/// (Lunar Insight-style "for each different mana value"). `you` resolves the
/// filter's controller constraints.
pub fn distinct_mana_values(state: &GameState, filter: &ObjectFilter, you: PlayerId) -> u32 {
    let mut set: std::collections::HashSet<u32> = Default::default();
    for o in state.objects.objects_in_zone(Zone::Battlefield) {
        if filter.matches(o, state, you) { set.insert(o.characteristics.mana_value()); }
    }
    set.len() as u32
}

/// Creatures (and planeswalkers — both emit `Dies`) that died this turn while
/// `player` controlled them. The controller-scoped variant of
/// [`creatures_died_this_turn`] — for "for each creature YOU CONTROL that died
/// this turn" (Fresh Meat-class), which the all-player count over-states.
/// Controller-at-death is read from LKI (CR 603.10) when the object has moved.
pub fn creatures_died_this_turn_controlled_by(state: &GameState, player: PlayerId) -> u32 {
    if !valid(state, player) { return 0; }
    this_turn_events(state).iter().filter(|ev| match ev {
        crate::events::GameEvent::Dies { object_id } => {
            state.objects.get(*object_id).or_else(|| state.lki.get(object_id))
                .is_some_and(|o| o.controller == player)
        }
        _ => false,
    }).count() as u32
}

/// The IDs (not just the count) of cards in `player`'s graveyard matching
/// `filter`, stable order — feed into `Effect::ForEach` for "return ALL X from
/// your graveyard" (Wake the Past-class). Sibling of [`graveyard_matching`].
pub fn graveyard_ids_matching(
    state: &GameState, filter: &ObjectFilter, player: PlayerId, you: PlayerId,
) -> Vec<ObjectId> {
    if !valid(state, player) { return Vec::new(); }
    state.objects.objects_in_zone(Zone::Graveyard(player))
        .filter(|o| filter.matches(o, state, you))
        .map(|o| o.id)
        .collect()
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
    fn devotion_counts_colored_pips_among_your_permanents() {
        use crate::mana::ManaCost;
        let mut s = GameState::new(2, 0);
        let with_cost = |cost: &str| Characteristics {
            mana_cost: Some(ManaCost::parse(cost).unwrap()),
            types: TypeLine::CREATURE.into(),
            ..Default::default()
        };
        // You control: {G}{G} (2 green pips), {1}{G} (1 green pip),
        // {W}{U} (0 green). Opponent controls {G}{G} (doesn't count).
        put(&mut s, Zone::Battlefield, 0, with_cost("{G}{G}"));
        put(&mut s, Zone::Battlefield, 0, with_cost("{1}{G}"));
        put(&mut s, Zone::Battlefield, 0, with_cost("{W}{U}"));
        put(&mut s, Zone::Battlefield, 1, with_cost("{G}{G}"));

        assert_eq!(devotion(&s, 0, ColorSet::green()), 3,
            "2 + 1 green pips among your permanents (opponent's excluded)");
        assert_eq!(devotion(&s, 0, ColorSet::white()), 1);
        // Devotion to white-or-blue counts each qualifying symbol once.
        assert_eq!(devotion(&s, 0, ColorSet::white() | ColorSet::blue()), 2);
        assert_eq!(devotion(&s, 0, ColorSet::red()), 0);
        assert_eq!(devotion(&s, 99, ColorSet::green()), 0, "invalid player → 0");
    }

    #[test]
    fn devotion_counts_hybrid_pips_for_each_named_color() {
        use crate::mana::ManaCost;
        let mut s = GameState::new(2, 0);
        // A {G/W} hybrid pip counts for green devotion AND white devotion.
        put(&mut s, Zone::Battlefield, 0, Characteristics {
            mana_cost: Some(ManaCost::parse("{G/W}").unwrap()),
            types: TypeLine::CREATURE.into(),
            ..Default::default()
        });
        assert_eq!(devotion(&s, 0, ColorSet::green()), 1);
        assert_eq!(devotion(&s, 0, ColorSet::white()), 1);
        assert_eq!(devotion(&s, 0, ColorSet::black()), 0);
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
    fn source_counter_count_reads_the_source_object() {
        use crate::types::CounterKind;
        let mut s = GameState::new(2, 0);
        let src = put(&mut s, Zone::Battlefield, 0, Characteristics::default());
        let lore = CounterKind::Lore;
        assert_eq!(source_counter_count(&s, src, lore), 0);
        s.objects.get_mut(src).unwrap().add_counters(lore, 3);
        assert_eq!(source_counter_count(&s, src, lore), 3);
        // Other kinds are independent; absent source → 0.
        assert_eq!(source_counter_count(&s, src, CounterKind::PlusOnePlusOne), 0);
        assert_eq!(source_counter_count(&s, 99_999, lore), 0);
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

    // --- Tier 1 accessors -------------------------------------------------

    #[test]
    fn name_of_returns_the_interned_handle_for_same_name_matching() {
        use crate::registry::CardRegistry;
        let mut reg = CardRegistry::new();
        let llanowar = reg.interner_mut().intern("Llanowar Elves");
        let mut s = GameState::new(2, 0);
        let named = Characteristics {
            name: llanowar, types: TypeLine::CREATURE.into(),
            ..Default::default()
        };
        let id = put(&mut s, Zone::Battlefield, 0, named);
        // Two objects of the same name share one handle ⇒ name equality is
        // handle equality (Doubling Chant-class "same name" matching).
        let id2 = put(&mut s, Zone::Library(0), 0, Characteristics {
            name: llanowar, ..Default::default()
        });
        assert_eq!(name_of(&s, id), Some(llanowar));
        assert_eq!(name_of(&s, id), name_of(&s, id2));
        assert_eq!(name_of(&s, 99_999), None, "absent object → None, no panic");
    }

    #[test]
    fn distinct_card_types_in_graveyard_counts_categories_once() {
        let mut s = GameState::new(2, 0);
        let chars = |tl: TypeLine| Characteristics { types: tl, ..Default::default() };
        // Two creatures (one category), one instant, one land ⇒ 3 distinct.
        put(&mut s, Zone::Graveyard(0), 0, chars(TypeLine::CREATURE.into()));
        put(&mut s, Zone::Graveyard(0), 0, chars(TypeLine::CREATURE.into()));
        put(&mut s, Zone::Graveyard(0), 0, chars(TypeLine::INSTANT.into()));
        put(&mut s, Zone::Graveyard(0), 0, chars(TypeLine::LAND.into()));
        // A multi-type card contributes each of its categories.
        put(&mut s, Zone::Graveyard(0), 0, chars(
            TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)));
        // Opponent's graveyard is independent.
        put(&mut s, Zone::Graveyard(1), 1, chars(TypeLine::SORCERY.into()));
        assert_eq!(distinct_card_types_in_graveyard(&s, 0), 4,
            "creature + instant + land + artifact");
        assert_eq!(distinct_card_types_in_graveyard(&s, 1), 1);
        assert_eq!(distinct_card_types_in_graveyard(&s, 99), 0, "invalid → 0");
    }

    #[test]
    fn distinct_mana_values_dedupes_across_matching_permanents() {
        use crate::mana::ManaCost;
        let mut s = GameState::new(2, 0);
        let cre = |cost: &str| Characteristics {
            mana_cost: Some(ManaCost::parse(cost).unwrap()),
            types: TypeLine::CREATURE.into(),
            ..Default::default()
        };
        // mv 1, 2, 3, and a duplicate mv 1 ⇒ 3 distinct.
        put(&mut s, Zone::Battlefield, 0, cre("{G}"));
        put(&mut s, Zone::Battlefield, 0, cre("{1}{G}"));
        put(&mut s, Zone::Battlefield, 0, cre("{2}{G}"));
        put(&mut s, Zone::Battlefield, 0, cre("{W}"));
        // A land (no cost ⇒ mv 0) is excluded by the creature filter.
        put(&mut s, Zone::Battlefield, 0, Characteristics {
            types: TypeLine::LAND.into(), ..Default::default() });
        assert_eq!(distinct_mana_values(&s, &ObjectFilter::creature(), 0), 3);
    }

    #[test]
    fn creatures_died_controlled_by_is_per_controller() {
        let mut s = GameState::new(2, 0);
        // p0 controls two dying creatures, p1 controls one.
        let a = put(&mut s, Zone::Battlefield, 0, creature_chars(1, 1));
        let b = put(&mut s, Zone::Battlefield, 0, creature_chars(1, 1));
        let c = put(&mut s, Zone::Battlefield, 1, creature_chars(1, 1));
        // Snapshot into LKI to mimic post-death lookup.
        for id in [a, b, c] {
            let obj = s.objects.get(id).unwrap().clone();
            s.lki.insert(id, obj);
        }
        s.turn_event_log_start = 0;
        s.emit(crate::events::GameEvent::Dies { object_id: a });
        s.emit(crate::events::GameEvent::Dies { object_id: b });
        s.emit(crate::events::GameEvent::Dies { object_id: c });
        assert_eq!(creatures_died_this_turn_controlled_by(&s, 0), 2,
            "Fresh Meat-class: only creatures YOU controlled");
        assert_eq!(creatures_died_this_turn_controlled_by(&s, 1), 1);
        // The all-player count is the (larger) sum.
        assert_eq!(creatures_died_this_turn(&s), 3);
        assert_eq!(creatures_died_this_turn_controlled_by(&s, 99), 0);
    }

    #[test]
    fn graveyard_ids_matching_returns_the_matching_ids() {
        let mut s = GameState::new(2, 0);
        let cre = put(&mut s, Zone::Graveyard(0), 0, creature_chars(2, 2));
        put(&mut s, Zone::Graveyard(0), 0, Characteristics {
            types: TypeLine::INSTANT.into(), ..Default::default() });
        let ids = graveyard_ids_matching(&s, &ObjectFilter::creature(), 0, 0);
        assert_eq!(ids, vec![cre], "only the creature card's id");
        // Count and id-list agree.
        assert_eq!(ids.len() as u32,
            graveyard_matching(&s, &ObjectFilter::creature(), 0, 0));
        assert!(graveyard_ids_matching(&s, &ObjectFilter::creature(), 99, 0).is_empty());
    }

    #[test]
    fn commodity_subtype_filter_matches_by_kind_marker() {
        use crate::registry::CardRegistry;
        use crate::effects::CommodityToken;
        let reg = CardRegistry::new(); // no cards interned — works anyway
        let mut s = GameState::new(2, 0);
        let art = || Characteristics {
            types: TypeLine::ARTIFACT.into(), ..Default::default() };
        // A Treasure, a Clue, and a plain (unmarked) artifact.
        let treasure = put(&mut s, Zone::Battlefield, 0, art());
        s.objects.get_mut(treasure).unwrap().commodity = Some(CommodityToken::Treasure);
        let clue = put(&mut s, Zone::Battlefield, 0, art());
        s.objects.get_mut(clue).unwrap().commodity = Some(CommodityToken::Clue);
        put(&mut s, Zone::Battlefield, 0, art()); // plain artifact, no marker

        // Each commodity name matches only its own kind; an unminted
        // commodity (Food) matches nothing; the plain artifact is no Treasure.
        assert_eq!(count_matching(&s, &subtype_filter(&reg, "Treasure"), 0), 1);
        assert_eq!(count_matching(&s, &subtype_filter(&reg, "Clue"), 0), 1);
        assert_eq!(count_matching(&s, &subtype_filter(&reg, "Food"), 0), 0);
        assert_eq!(ids_matching(&s, &subtype_filter(&reg, "Treasure"), 0), vec![treasure]);

        assert_eq!(commodity_kind_by_name("Treasure"), Some(CommodityToken::Treasure));
        assert_eq!(commodity_kind_by_name("Map"), Some(CommodityToken::Map));
        assert!(commodity_kind_by_name("Goblin").is_none(),
            "ordinary subtypes fall through to interner lookup");
    }
}
