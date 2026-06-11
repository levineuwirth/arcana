//! Triumph of Ferocity — `{2}{G}` enchantment.
//! "At the beginning of your upkeep, draw a card if you control the
//! creature with the greatest power or tied for the greatest power."

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
    let name = reg.interner_mut().intern("Triumph of Ferocity");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                effect: draw_if_greatest_power,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…draw a card if you control the creature with the greatest power or
/// tied for the greatest power." The greatest-power comparison is a
/// resolution-time condition ("draw a card if …"), checked here.
fn draw_if_greatest_power(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mine = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let all = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    let my_max = mine.iter().map(|&id| script::power_of(state, id)).max();
    let all_max = all.iter().map(|&id| script::power_of(state, id)).max();
    match (my_max, all_max) {
        (Some(m), Some(a)) if m >= a => vec![Effect::DrawCards {
            player: trig.controller,
            count: 1,
        }],
        _ => Vec::new(),
    }
}
