//! Serenity — `{1}{W}` enchantment.
//! "At the beginning of your upkeep, destroy all artifacts and
//! enchantments. They can't be regenerated."
//!
//! An upkeep sweep via `Effect::ForEach` over every artifact or
//! enchantment on the battlefield (including Serenity itself).
//! GAP: the "can't be regenerated" rider has no field on
//! `DestroyPermanent`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Serenity");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: sweep_artifacts_enchantments,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…destroy all artifacts and enchantments. They can't be
/// regenerated."
fn sweep_artifacts_enchantments(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::new().with_types_any(TypeLine(
        TypeLine::ARTIFACT | TypeLine::ENCHANTMENT,
    ));
    let ids = script::ids_matching(state, &filter, trig.controller);
    // GAP: "They can't be regenerated" — DestroyPermanent carries no
    // no-regeneration rider.
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: arcana_core::objects::NULL_OBJECT_ID,
        }),
    }]
}
