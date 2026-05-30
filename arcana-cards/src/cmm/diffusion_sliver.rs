//! Diffusion Sliver — `{1}{U}` 1/1 blue Sliver.
//! "Whenever a Sliver creature you control becomes the target of a spell
//! or ability an opponent controls, counter that spell or ability unless
//! its controller pays {2}."
//! GAP: trigger condition — "Whenever a Sliver you control becomes the
//! target of a spell or ability an opponent controls" requires a
//! ZoneChange-filtered SelfBecomesTarget variant scoped to any Sliver
//! you control, not just this card. The closest available is
//! SelfBecomesTarget which only fires for THIS creature. Mapped to
//! SelfBecomesTarget with caster Opponent as best-effort.
//! GAP: counter-a-spell effect is not in the Effect catalog; the
//! OptionalPayment "unless controller pays {2}" shell fires instead
//! and the counter action is omitted.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Diffusion Sliver");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sliver);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                // GAP: trigger should fire for any Sliver you control being
                // targeted, not just this card. Using SelfBecomesTarget as
                // best-effort approximation.
                trigger_condition: TriggerCondition::SelfBecomesTarget {
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: counter_unless_pays,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn counter_unless_pays(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: counter the targeting spell/ability is not in the Effect catalog.
    // The "unless controller pays {2}" OptionalPayment shell is wired with
    // a no-op on pay; the actual counter effect is omitted.
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: None,
    }]
}
