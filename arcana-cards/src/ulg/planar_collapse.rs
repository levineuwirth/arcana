//! Planar Collapse — `{1}{W}` enchantment (Urza's Legacy, 1999).
//! "At the beginning of your upkeep, if there are four or more creatures on
//! the battlefield, sacrifice this enchantment and destroy all creatures.
//! They can't be regenerated."
//!
//! Upkeep trigger with an intervening-if counting ALL creatures on the
//! battlefield (any controller). Fidelity GAPs: "sacrifice this
//! enchantment" is modeled as destroying it (no sacrifice-self one-shot),
//! and "they can't be regenerated" is unmodeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Planar Collapse");
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
                intervening_if: Some(if_four_or_more_creatures),
                effect: collapse,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…if there are four or more creatures on the battlefield…" — counts all
/// creatures regardless of controller.
fn if_four_or_more_creatures(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::count_matching(s, &ObjectFilter::creature(), you) >= 4
}

/// "…sacrifice this enchantment and destroy all creatures. They can't be
/// regenerated."
fn collapse(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let ids =
        script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    vec![
        // GAP: "sacrifice this enchantment" — no sacrifice-self one-shot
        // effect; destroying it is the closest (regeneration-window
        // fidelity gap).
        Effect::DestroyPermanent {
            target: trig.source,
        },
        // GAP: "They can't be regenerated" is unmodeled — plain destroy.
        Effect::ForEach {
            targets: ids,
            effect: Box::new(Effect::DestroyPermanent {
                target: arcana_core::objects::NULL_OBJECT_ID,
            }),
        },
    ]
}
