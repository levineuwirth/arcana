//! Intervening-if condition prelude (CR 603.4).
//!
//! A small, **total, panic-free, read-only** set of boolean predicates
//! for [`crate::triggers::InterveningIfFn`] bodies — the sibling of
//! [`crate::script`] (which returns resolution-time *amounts*; these
//! return the *bool* a trigger's `intervening_if` needs).
//!
//! A generated card wires an intervening-if by writing a tiny named
//! `fn(&GameState, ObjectId, PlayerId, &CardRegistry) -> bool` (state,
//! source, controller, registry — the registry trailing, matching
//! [`crate::triggers::EffectFn`]) that calls one of these, then setting
//! `intervening_if: Some(that_fn)` on its [`crate::triggers::TriggeredAbilityDef`]:
//!
//! ```ignore
//! fn if_control_three_artifacts(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
//!     conditions::you_control_at_least(
//!         s, you,
//!         &ObjectFilter { types: Some(TypeLine::ARTIFACT.into()), ..Default::default() },
//!         3,
//!     )
//! }
//! ```
//!
//! Conditions that name a SUBTYPE ("if you control two or more Gates",
//! "if you control a Chandra planeswalker") need the registry's interner
//! to resolve the name to a [`crate::types::SmallString`] id — that's
//! why the predicate receives `&CardRegistry`. Use
//! [`you_control_subtype`] / [`you_control_subtype_at_least`], which do
//! the lookup (and answer `false` for a never-interned name, since you
//! can't control a permanent of a subtype no card has introduced):
//!
//! ```ignore
//! fn if_two_or_more_gates(s: &GameState, _src: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
//!     conditions::you_control_subtype_at_least(s, reg, you, "Gate", 2)
//! }
//! ```
//!
//! Same defensive contract as [`crate::script`]: an invalid player or
//! missing object yields the neutral answer (`false` / `0`), never a
//! panic.

use crate::objects::ObjectId;
use crate::registry::CardRegistry;
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

/// "if an opponent controls a/an [filter]" — any battlefield permanent
/// controlled by a player other than `you` that matches. The filter's
/// controller constraints (if any) still resolve against `you` as the
/// source controller, consistent with [`you_control_a`].
pub fn an_opponent_controls_a(state: &GameState, you: PlayerId, filter: &ObjectFilter) -> bool {
    state
        .objects
        .objects_in_zone(Zone::Battlefield)
        .any(|o| o.controller != you && filter.matches(o, state, you))
}

/// "if a/an [filter] is on the battlefield" — battlefield-wide,
/// regardless of controller (any controller constraint inside the
/// filter still resolves against `you`). Pair with
/// [`crate::targets::ObjectFilter::attacking_only`]-style combat
/// builders for "if a Rat is attacking"-shaped conditions.
pub fn a_permanent_matches(state: &GameState, you: PlayerId, filter: &ObjectFilter) -> bool {
    state
        .objects
        .objects_in_zone(Zone::Battlefield)
        .any(|o| filter.matches(o, state, you))
}

// --- This-turn event history (Boast / morbid / bloodthirst-adjacent) --
// Thin wrappers over the `script::*_this_turn` event-log scanners.

/// Boast and friends — "activate only if this creature attacked this
/// turn" / intervening-if "if ~ attacked this turn".
pub fn source_attacked_this_turn(state: &GameState, source: ObjectId) -> bool {
    crate::script::creature_attacked_this_turn(state, source)
}

/// "if you attacked this turn" (any creature you control was declared
/// as an attacker).
pub fn you_attacked_this_turn(state: &GameState, you: PlayerId) -> bool {
    crate::script::player_attacked_this_turn(state, you)
}

/// Morbid — "if a creature died this turn" (any controller).
pub fn a_creature_died_this_turn(state: &GameState) -> bool {
    crate::script::creatures_died_this_turn(state) >= 1
}

/// "if an opponent lost life this turn" — the MID/VOW Vampire gate.
/// Life lost includes damage dealt to the player (CR 120.3).
pub fn an_opponent_lost_life_this_turn(state: &GameState, you: PlayerId) -> bool {
    (0..state.num_players())
        .any(|p| p != you && crate::script::life_lost_this_turn(state, p) >= 1)
}

/// "if you gained life this turn".
pub fn you_gained_life_this_turn(state: &GameState, you: PlayerId) -> bool {
    crate::script::life_gained_this_turn(state, you) >= 1
}

/// "if you've cast a/an [filter] spell this turn" — e.g. noncreature:
/// `ObjectFilter::new().without_types(TypeLine::CREATURE.into())`.
/// The filter is matched against the cast spell's stack object (arena
/// or LKI); controller is hard-scoped to `you`.
pub fn you_cast_matching_this_turn(
    state: &GameState,
    you: PlayerId,
    filter: &ObjectFilter,
) -> bool {
    let scoped = ObjectFilter {
        controller: Some(crate::targets::ControllerConstraint::You),
        ..filter.clone()
    };
    crate::script::spells_cast_this_turn(state, &scoped, you) >= 1
}

/// "if a [filter] entered the battlefield this turn" — e.g. "a
/// creature entered under your control this turn" =
/// `ObjectFilter::creature().controlled_by(ControllerConstraint::You)`.
pub fn entered_this_turn(state: &GameState, you: PlayerId, filter: &ObjectFilter) -> bool {
    crate::script::entered_this_turn_matching(state, filter, you) >= 1
}

/// "if you control N or more permanents with subtype `subtype`" — the
/// name is resolved to its [`crate::types::SmallString`] id via the
/// registry's interner (subtype ids are dynamic, so an intervening-if
/// `fn` can't bake one in; it gets `&CardRegistry` for exactly this).
/// A name that was never interned can't be on any permanent, so the
/// answer is `false` (you control zero) rather than a panic. Matches
/// any card type — "Gate" only appears on lands, "Chandra" only on
/// planeswalkers, so a bare subtype check is sufficient; combine with a
/// type filter manually if a subtype is shared across types.
pub fn you_control_subtype_at_least(
    state: &GameState,
    reg: &CardRegistry,
    you: PlayerId,
    subtype: &str,
    n: u32,
) -> bool {
    match reg.interner().lookup(subtype) {
        Some(sym) => {
            count_you_control(state, you, &ObjectFilter::new().with_subtype_sym(sym)) >= n
        }
        None => false,
    }
}

/// "if you control a/an [subtype]" (one or more) — see
/// [`you_control_subtype_at_least`].
pub fn you_control_subtype(
    state: &GameState,
    reg: &CardRegistry,
    you: PlayerId,
    subtype: &str,
) -> bool {
    you_control_subtype_at_least(state, reg, you, subtype, 1)
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

/// "if there are N or more cards in your graveyard" (threshold).
pub fn graveyard_at_least(state: &GameState, you: PlayerId, n: u32) -> bool {
    crate::script::graveyard_size(state, you) >= n
}

/// "if there are N or more [filter] cards in your graveyard" — the
/// filtered sibling of [`graveyard_at_least`] ("three or more creature
/// cards", "three or more instant and/or sorcery cards", "six or more
/// permanent cards"). Build the filter from the card types the clause
/// names; a "permanent card" is any of artifact/creature/enchantment/
/// land/planeswalker/battle, expressed with `with_types_any`.
pub fn graveyard_matching_at_least(
    state: &GameState,
    you: PlayerId,
    filter: &ObjectFilter,
    n: u32,
) -> bool {
    crate::script::graveyard_matching(state, filter, you, you) >= n
}

/// "if there is a [filter] card in your graveyard" (one or more).
pub fn graveyard_has(state: &GameState, you: PlayerId, filter: &ObjectFilter) -> bool {
    graveyard_matching_at_least(state, you, filter, 1)
}

/// "if there are N or more cards with subtype `subtype` in your
/// graveyard" — resolves the name via the registry interner (the
/// graveyard sibling of [`you_control_subtype_at_least`]; `false` for a
/// never-interned name). For "an Elf card", "a Lesson card", "a Desert
/// card in your graveyard".
pub fn graveyard_subtype_at_least(
    state: &GameState,
    reg: &CardRegistry,
    you: PlayerId,
    subtype: &str,
    n: u32,
) -> bool {
    match reg.interner().lookup(subtype) {
        Some(sym) => crate::script::graveyard_matching(
            state,
            &ObjectFilter::new().with_subtype_sym(sym),
            you,
            you,
        ) >= n,
        None => false,
    }
}

/// "if there is a [subtype] card in your graveyard" — see
/// [`graveyard_subtype_at_least`].
pub fn graveyard_has_subtype(
    state: &GameState,
    reg: &CardRegistry,
    you: PlayerId,
    subtype: &str,
) -> bool {
    graveyard_subtype_at_least(state, reg, you, subtype, 1)
}

/// CR 702.84 — the number of distinct CARD TYPES among cards in your
/// graveyard (artifact, battle, creature, enchantment, instant, kindred,
/// land, planeswalker, sorcery). The raw count behind [`delirium`].
pub fn graveyard_card_type_count(state: &GameState, you: PlayerId) -> u32 {
    use crate::types::TypeLine;
    const CARD_TYPES: u16 = TypeLine::CREATURE
        | TypeLine::INSTANT
        | TypeLine::SORCERY
        | TypeLine::ENCHANTMENT
        | TypeLine::ARTIFACT
        | TypeLine::LAND
        | TypeLine::PLANESWALKER
        | TypeLine::KINDRED
        | TypeLine::BATTLE;
    if !valid(state, you) {
        return 0;
    }
    let union = state
        .objects
        .objects_in_zone(Zone::Graveyard(you))
        .fold(0u16, |acc, o| acc | o.characteristics.types.0);
    (union & CARD_TYPES).count_ones()
}

/// "if there are four or more card types among cards in your graveyard"
/// (delirium).
pub fn delirium(state: &GameState, you: PlayerId) -> bool {
    graveyard_card_type_count(state, you) >= 4
}

/// "if an opponent has N or more cards in their graveyard" — true if ANY
/// player other than `you` meets the threshold.
pub fn an_opponent_graveyard_at_least(state: &GameState, you: PlayerId, n: u32) -> bool {
    (0..state.num_players()).any(|p| p != you && crate::script::graveyard_size(state, p) >= n)
}

/// "if [this permanent]'s power is N or greater" — reads the source's
/// current power after the layer system ("activate only if this
/// creature's power is 4 or greater"). A missing/non-creature source
/// reads as power 0.
pub fn source_power_at_least(state: &GameState, source: ObjectId, n: i32) -> bool {
    crate::script::power_of(state, source) >= n
}

/// "Activate only during your turn, before attackers are declared"
/// (CR 602.5e timing window — the Portal/PTK tap-creature cycle, Loyal
/// Retainers, Norwood Priestess). True while it is `you`r turn AND the
/// turn has not yet reached the declare-attackers step: any beginning
/// step, first main, or begin-combat qualifies; declare-attackers
/// onward (and the post-combat phases) do not. Extra combat phases are
/// treated like the first — once any declare-attackers step has begun,
/// the window stays shut for the rest of the turn.
pub fn your_turn_before_attackers(state: &GameState, you: PlayerId) -> bool {
    use crate::turn::{Phase, Step};
    if state.active_player() != you { return false; }
    match state.turn.phase {
        Phase::Beginning | Phase::PreCombatMain => true,
        Phase::Combat => state.turn.step == Step::BeginCombat,
        Phase::PostCombatMain | Phase::Ending => false,
    }
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
    fn your_turn_before_attackers_window() {
        use crate::turn::{Phase, Step};
        let mut s = GameState::new(2, 0);
        // Player 0's turn, upkeep: open for 0, shut for 1.
        s.turn.phase = Phase::Beginning;
        s.turn.step = Step::Upkeep;
        assert!( your_turn_before_attackers(&s, 0));
        assert!(!your_turn_before_attackers(&s, 1));
        // First main: open.
        s.turn.phase = Phase::PreCombatMain;
        s.turn.step = Step::Main;
        assert!(your_turn_before_attackers(&s, 0));
        // Begin combat: still open (attackers not yet declared).
        s.turn.phase = Phase::Combat;
        s.turn.step = Step::BeginCombat;
        assert!(your_turn_before_attackers(&s, 0));
        // Declare attackers onward: shut.
        s.turn.step = Step::DeclareAttackers;
        assert!(!your_turn_before_attackers(&s, 0));
        s.turn.phase = Phase::PostCombatMain;
        s.turn.step = Step::Main;
        assert!(!your_turn_before_attackers(&s, 0));
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

    #[test]
    fn subtype_predicate_resolves_name_via_interner() {
        use crate::types::{SubtypeSet, TypeLine};
        let mut reg = CardRegistry::new();
        let gate = reg.interner_mut().intern("Gate");
        let gate_land = || {
            let mut subs = SubtypeSet::default();
            subs.0.insert(gate);
            Characteristics { types: TypeLine::LAND.into(), subtypes: subs, ..Default::default() }
        };
        let mut s = GameState::new(2, 0);
        // No Gates yet → false.
        assert!(!you_control_subtype(&s, &reg, 0, "Gate"));
        // Two Gates you control + one the opponent controls.
        put(&mut s, 0, gate_land());
        put(&mut s, 0, gate_land());
        put(&mut s, 1, gate_land()); // opponent's — must not count for you
        assert!(you_control_subtype(&s, &reg, 0, "Gate"));
        assert!(you_control_subtype_at_least(&s, &reg, 0, "Gate", 2));
        assert!(!you_control_subtype_at_least(&s, &reg, 0, "Gate", 3));
        // Opponent controls only one.
        assert!(!you_control_subtype_at_least(&s, &reg, 1, "Gate", 2));
        // A never-interned subtype can't be on any permanent → false, no panic.
        assert!(!you_control_subtype(&s, &reg, 0, "Sliver"));
    }

    #[test]
    fn graveyard_filter_delirium_and_opponent_predicates() {
        use crate::objects::GameObject;
        use crate::types::{SubtypeSet, TypeLine};
        let mut reg = CardRegistry::new();
        let elf = reg.interner_mut().intern("Elf");
        let mut s = GameState::new(2, 0);
        let put_gy = |s: &mut GameState, owner: PlayerId, types: TypeLine, subs: SubtypeSet| {
            let id = s.allocate_object_id();
            let chars = Characteristics { types, subtypes: subs, ..Default::default() };
            let mut o = GameObject::new(id, owner, Zone::Graveyard(owner), 0, chars);
            o.controller = owner;
            s.objects.insert(o);
        };
        let none = SubtypeSet::default();
        let mut elf_sub = SubtypeSet::default();
        elf_sub.0.insert(elf);
        // Your graveyard: 3 creature cards (one an Elf), 1 instant, 1 land.
        put_gy(&mut s, 0, TypeLine::CREATURE.into(), elf_sub);
        put_gy(&mut s, 0, TypeLine::CREATURE.into(), none.clone());
        put_gy(&mut s, 0, TypeLine::CREATURE.into(), none.clone());
        put_gy(&mut s, 0, TypeLine::INSTANT.into(), none.clone());
        put_gy(&mut s, 0, TypeLine::LAND.into(), none.clone());
        let creatures = ObjectFilter::new().with_types(TypeLine::CREATURE.into());
        assert!(graveyard_matching_at_least(&s, 0, &creatures, 3));
        assert!(!graveyard_matching_at_least(&s, 0, &creatures, 4));
        assert!(graveyard_has(&s, 0, &creatures));
        // Subtype-in-graveyard via the interner.
        assert!(graveyard_has_subtype(&s, &reg, 0, "Elf"));
        assert!(!graveyard_has_subtype(&s, &reg, 0, "Goblin"));
        // Delirium: creature + instant + land = 3 types → not yet.
        assert_eq!(graveyard_card_type_count(&s, 0), 3);
        assert!(!delirium(&s, 0));
        put_gy(&mut s, 0, TypeLine::ENCHANTMENT.into(), none.clone());
        assert!(delirium(&s, 0)); // four distinct card types now
        // Opponent graveyard threshold.
        assert!(!an_opponent_graveyard_at_least(&s, 0, 1));
        put_gy(&mut s, 1, TypeLine::SORCERY.into(), none);
        assert!(an_opponent_graveyard_at_least(&s, 0, 1));
        assert!(!an_opponent_graveyard_at_least(&s, 0, 2));
    }

    #[test]
    fn opponent_controls_predicate() {
        use crate::effects::KeywordAbility;
        let mut s = GameState::new(2, 0);
        let put = |s: &mut GameState, controller: PlayerId, flying: bool| {
            let id = s.allocate_object_id();
            let mut chars = Characteristics {
                types: TypeLine::CREATURE.into(),
                ..Default::default()
            };
            if flying { chars.keywords.push(KeywordAbility::Flying); }
            let mut o = GameObject::new(id, controller, Zone::Battlefield, 0, chars);
            o.controller = controller;
            s.objects.insert(o);
        };
        let flyers = ObjectFilter::creature().with_keyword(KeywordAbility::Flying);
        // Your own flyer doesn't satisfy "an opponent controls".
        put(&mut s, 0, true);
        assert!(!an_opponent_controls_a(&s, 0, &flyers));
        // An opponent's grounded creature doesn't either.
        put(&mut s, 1, false);
        assert!(!an_opponent_controls_a(&s, 0, &flyers));
        // An opponent's flyer does.
        put(&mut s, 1, true);
        assert!(an_opponent_controls_a(&s, 0, &flyers));
    }

    #[test]
    fn this_turn_event_history_predicates() {
        use crate::combat::DefendingEntity;
        use crate::events::GameEvent;
        let mut s = GameState::new(2, 0);
        let attacker = put(&mut s, 0, creature(2, 2));
        let bystander = put(&mut s, 0, creature(2, 2));

        // Stale history from an earlier turn must NOT count.
        s.event_log.push(GameEvent::CreatureAttacks {
            attacker, defending: DefendingEntity::Player(1),
        });
        s.event_log.push(GameEvent::Dies { object_id: bystander });
        s.event_log.push(GameEvent::LifeLost { player: 1, amount: 3 });
        s.turn_event_log_start = s.event_log.len();

        assert!(!source_attacked_this_turn(&s, attacker));
        assert!(!you_attacked_this_turn(&s, 0));
        assert!(!a_creature_died_this_turn(&s));
        assert!(!an_opponent_lost_life_this_turn(&s, 0));
        assert!(!you_gained_life_this_turn(&s, 0));

        // Live-turn events flip each predicate.
        s.event_log.push(GameEvent::CreatureAttacks {
            attacker, defending: DefendingEntity::Player(1),
        });
        assert!(source_attacked_this_turn(&s, attacker));
        assert!(!source_attacked_this_turn(&s, bystander));
        assert!(you_attacked_this_turn(&s, 0));
        assert!(!you_attacked_this_turn(&s, 1));

        s.event_log.push(GameEvent::Dies { object_id: bystander });
        assert!(a_creature_died_this_turn(&s));

        s.event_log.push(GameEvent::LifeLost { player: 1, amount: 2 });
        assert!(an_opponent_lost_life_this_turn(&s, 0));
        assert!(!an_opponent_lost_life_this_turn(&s, 1)); // p0 lost none

        s.event_log.push(GameEvent::LifeGained { player: 0, amount: 1 });
        assert!(you_gained_life_this_turn(&s, 0));
        assert!(!you_gained_life_this_turn(&s, 1));

        // Entered-this-turn: the EntersBattlefield event marks it.
        let entrant = put(&mut s, 0, creature(1, 1));
        assert!(!entered_this_turn(&s, 0, &ObjectFilter::creature()));
        s.event_log.push(GameEvent::EntersBattlefield {
            object_id: entrant, from_zone: Zone::Hand(0), was_cast: false,
        });
        assert!(entered_this_turn(&s, 0, &ObjectFilter::creature()));
        assert!(crate::script::entered_battlefield_this_turn(&s, entrant));
        assert!(!crate::script::entered_battlefield_this_turn(&s, bystander));

        // Cast-matching: a noncreature (sorcery) stack object cast by p0.
        use crate::objects::GameObject;
        let spell_id = s.allocate_object_id();
        let chars = Characteristics {
            types: TypeLine::SORCERY.into(),
            ..Default::default()
        };
        let mut spell = GameObject::new(spell_id, 0, Zone::Stack, 9, chars);
        spell.controller = 0;
        s.objects.insert(spell);
        let noncreature = ObjectFilter::new()
            .without_types(TypeLine::CREATURE.into());
        assert!(!you_cast_matching_this_turn(&s, 0, &noncreature));
        s.event_log.push(GameEvent::SpellCast {
            object_id: spell_id, card_id: 9, controller: 0,
            targets: crate::targets::TargetSelection::new(),
        });
        assert!(you_cast_matching_this_turn(&s, 0, &noncreature));
        assert!(!you_cast_matching_this_turn(&s, 1, &noncreature));
    }
}
