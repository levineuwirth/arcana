//! Impending Disaster — `{1}{R}` enchantment (Urza's Legacy, 1999).
//! "At the beginning of your upkeep, if there are seven or more lands on
//! the battlefield, sacrifice this enchantment and destroy all lands."
//!
//! Upkeep trigger with an intervening-if counting ALL lands on the
//! battlefield (any controller). Fidelity GAP: "sacrifice this enchantment"
//! is modeled as destroying it (no sacrifice-self one-shot).

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
    let name = reg.interner_mut().intern("Impending Disaster");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                intervening_if: Some(if_seven_or_more_lands),
                effect: disaster,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…if there are seven or more lands on the battlefield…" — counts all
/// lands regardless of controller.
fn if_seven_or_more_lands(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    script::count_matching(
        s,
        &ObjectFilter::new().with_types(TypeLine::LAND.into()),
        you,
    ) >= 7
}

/// "…sacrifice this enchantment and destroy all lands."
fn disaster(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let lands = script::ids_matching(
        state,
        &ObjectFilter::new().with_types(TypeLine::LAND.into()),
        trig.controller,
    );
    vec![
        // GAP: "sacrifice this enchantment" — no sacrifice-self one-shot
        // effect; destroying it is the closest (regeneration-window
        // fidelity gap).
        Effect::DestroyPermanent {
            target: trig.source,
        },
        Effect::ForEach {
            targets: lands,
            effect: Box::new(Effect::DestroyPermanent {
                target: arcana_core::objects::NULL_OBJECT_ID,
            }),
        },
    ]
}
