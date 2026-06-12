//! True Conviction — `{3}{W}{W}{W}` enchantment. "Creatures you
//! control have double strike and lifelink."
//!
//! Implementation: an ETB trigger installs TWO controller-wide keyword
//! anthems (double strike + lifelink) with duration
//! `WhileSourceOnBattlefield`, which the layer-cleanup pipeline
//! auto-expires when the enchantment leaves.

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
    let name = reg.interner_mut().intern("True Conviction");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}{W}").expect("valid cost")),
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
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::keyword_anthem(
                trig.source,
                trig.controller,
                KeywordAbility::DoubleStrike,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::keyword_anthem(
                trig.source,
                trig.controller,
                KeywordAbility::Lifelink,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
