//! Nature's Revolt — `{3}{G}{G}` enchantment.
//! "All lands are 2/2 creatures that are still lands."
//!
//! Implementation: ETB trigger installs two continuous effects:
//! 1. filtered_add_type adds CREATURE to all lands (L4).
//! 2. filtered_set_base_pt sets base P/T to 2/2 for all lands (L7b).
//! Filter has no controller constraint — "all lands" globally.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nature's Revolt");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_install,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let land_filter = ObjectFilter {
        types: Some(TypeLine::LAND.into()),
        ..Default::default()
    };
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_add_type(
                trig.source,
                land_filter.clone(),
                TypeLine::CREATURE.into(),
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_set_base_pt(
                trig.source,
                land_filter,
                2,
                2,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
