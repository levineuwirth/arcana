//! Vampire Socialite — `{B}{R}` 2/2 Creature — Vampire Noble. Black/red.
//! Menace.
//! "When this creature enters, if an opponent lost life this turn, put a
//!  +1/+1 counter on each other Vampire you control."
//! "As long as an opponent lost life this turn, each other Vampire you
//!  control enters with an additional +1/+1 counter on it." (static —
//!  GAP'd).
//!
//! The ETB trigger is gated by an intervening-if ("if an opponent lost
//! life this turn"). The counter placement runs over the Vampires you
//! control; the engine has no source-exclusion filter, so "each OTHER"
//! is approximated as "each" (self also receives a counter) — a
//! documented fidelity deviation.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vampire Socialite");
    let vampire = reg.interner_mut().intern("Vampire");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    // GAP: "As long as an opponent lost life this turn, each other Vampire
    // you control enters with an additional +1/+1 counter on it." — a
    // conditional enters-with-counters static replacement; not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: Some(if_an_opponent_lost_life),
                effect: counters_on_vampires,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_an_opponent_lost_life(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::an_opponent_lost_life_this_turn(s, you)
}

fn counters_on_vampires(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP detail: "each OTHER Vampire" — no source-exclusion filter, so
    // this includes Vampire Socialite itself (minor fidelity deviation).
    let filter = script::subtype_filter(reg, "Vampire")
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .map(|id| Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        })
        .collect()
}
