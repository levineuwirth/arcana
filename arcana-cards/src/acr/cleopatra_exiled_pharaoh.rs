//! Cleopatra, Exiled Pharaoh — `{2}{B}{G}` 2/4 Legendary Human Noble.
//!
//! * "Allies — At the beginning of your end step, put a +1/+1 counter on each of
//!   up to two other target legendary creatures."
//! * "Betrayal — Whenever a legendary creature with counters on it dies, draw a
//!   card for each counter on it. You lose 2 life." — the "with counters on it"
//!   trigger filter and "for each counter on it" amount are approximated by
//!   counting +1/+1 counters on the dying creature (see GAP).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cleopatra, Exiled Pharaoh");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: counter_two_legendaries,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    ),
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .with_supertypes(SupertypeSet::new().with(SupertypeSet::LEGENDARY)),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: betrayal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counter_two_legendaries(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    trig.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
            _ => None,
        })
        .collect()
}

fn betrayal(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "for each counter on it" should count counters of ALL kinds; the engine
    // exposes only per-kind counts, so this approximates with +1/+1 counters.
    let Some(id) = trig.dying_object() else {
        return Vec::new();
    };
    let n = state
        .objects
        .get(id)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    vec![
        Effect::DrawCards {
            player: trig.controller,
            count: n,
        },
        Effect::LoseLife {
            player: trig.controller,
            amount: 2,
        },
    ]
}
