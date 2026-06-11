//! Soul Barrier — `{2}{U}` enchantment.
//! "Whenever an opponent casts a creature spell, this enchantment deals 2
//! damage to that player unless they pay {2}."

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Soul Barrier");
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
                    filter: Some(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        ..ObjectFilter::default()
                    }),
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: tax_the_caster,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…this enchantment deals 2 damage to that player unless they pay {2}."
fn tax_the_caster(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(p) = trig.triggering_caster() else {
        return Vec::new();
    };
    vec![Effect::OptionalPayment {
        chooser: p,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}").expect("valid cost")),
        then: Box::new(Effect::Sequence(vec![])),
        else_effect: Some(Box::new(Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: 2,
            source: trig.source,
        })),
    }]
}
