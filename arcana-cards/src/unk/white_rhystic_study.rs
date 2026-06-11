//! White Rhystic Study — `{2}{W}` enchantment (Mystery Booster
//! playtest card). "Whenever an opponent casts a spell, you may draw a
//! card unless that player pays {1}."

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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("White Rhystic Study");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: tax_or_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…you may draw a card unless that player pays {1}." The caster is
/// the chooser; the draw is the punishment in `else_effect`.
/// FIDELITY: your "may" on the draw is approximated as automatic.
fn tax_or_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(caster) = trig.triggering_caster() else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: caster,
        cost: OptionalPaymentKind::Mana(
            ManaCost::parse("{1}").expect("valid cost"),
        ),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::DrawCards {
            player: trig.controller,
            count: 1,
        })),
    }]
}
