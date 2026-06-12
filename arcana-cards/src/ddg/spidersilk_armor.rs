//! Spidersilk Armor — `{2}{G}` enchantment. "Creatures you control get
//! +0/+1 and have reach."
//!
//! Implementation: the ETB trigger installs TWO continuous effects —
//! a +0/+1 anthem and a Reach keyword anthem — each with
//! `Duration::WhileSourceOnBattlefield`.

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
    let name = reg.interner_mut().intern("Spidersilk Armor");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_install_anthems,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "creatures you control get +0/+1" and
/// "creatures you control have reach", both anchored to this
/// enchantment and lasting while it remains on the battlefield.
fn etb_install_anthems(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::anthem(
                trig.source,
                trig.controller,
                0,
                1,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::keyword_anthem(
                trig.source,
                trig.controller,
                KeywordAbility::Reach,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
