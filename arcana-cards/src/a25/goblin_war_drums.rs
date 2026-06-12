//! Goblin War Drums — `{2}{R}` enchantment. "Creatures you control have
//! menace. (They can't be blocked except by two or more creatures.)"
//!
//! Implementation: an ETB trigger installs a
//! `ContinuousEffect::keyword_anthem` granting `Menace` to creatures you
//! control with `Duration::WhileSourceOnBattlefield`; the layer-cleanup
//! pipeline auto-expires it when the enchantment leaves.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Goblin War Drums");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_menace,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "creatures you control have menace" anchored to
/// this enchantment, lasting until it leaves the battlefield.
fn etb_install_menace(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::keyword_anthem(
            trig.source,
            trig.controller,
            KeywordAbility::Menace,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
