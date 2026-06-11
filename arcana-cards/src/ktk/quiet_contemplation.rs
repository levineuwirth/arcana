//! Quiet Contemplation — `{2}{U}` enchantment (Khans of Tarkir, 2014).
//! "Whenever you cast a noncreature spell, you may pay {1}. If you do,
//! tap target creature an opponent controls and it doesn't untap
//! during its controller's next untap step."
//!
//! A filtered `SpellCast` trigger with an opponent-creature target;
//! the pay-{1} gate is `Effect::OptionalPayment`, and the
//! doesn't-untap rider is modeled with a stun counter (CR 122.1c).

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Quiet Contemplation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(
                        ObjectFilter::new()
                            .without_types(TypeLine::CREATURE.into()),
                    ),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: contemplate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…you may pay {1}. If you do, tap target creature an opponent
/// controls and it doesn't untap during its controller's next untap
/// step." (Doesn't-untap modeled as a stun counter.)
fn contemplate(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{1}").expect("valid cost"),
        ),
        then: Box::new(Effect::Sequence(vec![
            Effect::Tap { target: *id },
            Effect::AddCounters {
                target: *id,
                kind: CounterKind::Stun,
                count: 1,
            },
        ])),
        else_effect: None,
    }]
}
