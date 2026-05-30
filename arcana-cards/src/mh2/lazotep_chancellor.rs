//! Lazotep Chancellor — `{U}{B}` 1/3 blue/black Zombie Wizard.
//! "Whenever you discard a card, you may pay {1}. If you do, amass Zombies 2."
//! GAP: Amass is not implemented as an Effect variant. The optional-payment
//! wrapper is used to model the "you may pay {1}" gate, but the amass body
//! returns Vec::new() pending engine support.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lazotep Chancellor");
    let zombie = reg.interner_mut().intern("Zombie");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: arcana_core::targets::ControllerConstraint::You,
                },
                intervening_if: None,
                effect: maybe_amass,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn maybe_amass(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{1}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![
            // GAP: Amass Zombies 2 is not expressible with any catalog Effect
            // variant. Engine support for amass (choose/create Army token and
            // add +1/+1 counters) is pending.
        ])),
        else_effect: None,
    }]
}
