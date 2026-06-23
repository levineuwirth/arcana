//! A Tale for the Ages — `{1}{W}` enchantment. "Enchanted creatures
//! you control get +2/+2."
//!
//! Implementation: an ETB trigger installs a controller-scoped anthem
//! ("creatures you control get +2/+2") with
//! `Duration::WhileSourceOnBattlefield`, auto-expired by the
//! layer-cleanup pipeline when the enchantment leaves. The "enchanted"
//! qualifier on a standalone (non-Aura) enchantment resolves to "your
//! creatures" — the closest faithful reading the controller-scoped
//! anthem builder can express.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A Tale for the Ages");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_anthem,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::anthem(
            trig.source,
            trig.controller,
            2,
            2,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
