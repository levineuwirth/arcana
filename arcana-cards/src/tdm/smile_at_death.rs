//! Smile at Death — `{3}{W}{W}` enchantment.
//! "At the beginning of your upkeep, return up to two target creature
//! cards with power 2 or less from your graveyard to the battlefield.
//! Put a +1/+1 counter on each of those creatures."
//!
//! Fidelity note: the +1/+1 counter is addressed at the graveyard-card
//! id; if the engine re-ids the object on re-entry the counter half may
//! not land (known continuation limitation).

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
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Smile at Death");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: return_and_grow,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature().with_max_power(2),
                },
                count: TargetCount::UpTo(2),
                controller: None,
            }],
        }),
    )
}

/// "…return up to two target creature cards … to the battlefield. Put a
/// +1/+1 counter on each of those creatures."
fn return_and_grow(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    for choice in &trig.targets.targets {
        if let TargetChoice::Object(id) = choice {
            effects.push(Effect::ReturnFromGraveyardToBattlefield { target: *id });
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            });
        }
    }
    vec![Effect::Sequence(effects)]
}
