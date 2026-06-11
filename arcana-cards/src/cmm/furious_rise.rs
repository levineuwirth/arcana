//! Furious Rise — `{2}{R}` enchantment (Theros Beyond Death).
//! "At the beginning of your end step, if you control a creature with
//! power 4 or greater, exile the top card of your library. You may
//! play that card until you exile another card with this enchantment."

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Furious Rise");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_control_power_four),
                effect: rise_impulse,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…if you control a creature with power 4 or greater…"
fn if_control_power_four(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::you_control_a(
        s,
        you,
        &ObjectFilter::creature().with_min_power(4),
    )
}

/// "…exile the top card of your library. You may play that card until
/// you exile another card with this enchantment."
/// FIDELITY: ImpulseExile's permission lapses at end of turn rather
/// than lasting until the next exile with this enchantment.
fn rise_impulse(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile {
        player: trig.controller,
        count: 1,
    }]
}
