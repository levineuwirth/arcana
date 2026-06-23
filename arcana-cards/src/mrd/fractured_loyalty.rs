//! Fractured Loyalty — `{1}{R}` enchantment — Aura.
//! "Enchant creature. Whenever enchanted creature becomes the target of a
//!  spell or ability, that spell or ability's controller gains control of that
//!  creature."
//!
//! The trigger watches the HOST becoming the target of any spell or ability —
//! `AttachedCreatureDoes(SelfBecomesTarget { caster: Any })`, which substitutes
//! the host for the source. On fire we read the `BecomesTarget` event's
//! controller (the targeting spell/ability's controller) and `ChangeControl`
//! the host to them. This is a permanent control change each time it's
//! targeted (CR 303-style aura), so there is no leave-revert.

use arcana_core::effects::Effect;
use arcana_core::events::GameEvent;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fractured Loyalty");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfBecomesTarget {
                        caster: ControllerConstraint::Any,
                    }),
                },
                intervening_if: None,
                effect: host_targeted_gain_control,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn host_targeted_gain_control(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    match trig.trigger_event {
        GameEvent::BecomesTarget { target, controller, .. } => {
            vec![Effect::ChangeControl {
                target,
                new_controller: controller,
            }]
        }
        _ => Vec::new(),
    }
}
