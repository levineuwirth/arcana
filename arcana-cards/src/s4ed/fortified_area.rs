//! Fortified Area — `{1}{W}{W}` enchantment. "Wall creatures you control
//! get +1/+0 and have banding."
//!
//! Implementation: an ETB trigger installs TWO continuous effects with
//! [`Duration::WhileSourceOnBattlefield`] — a
//! [`ContinuousEffect::filtered_pump`] (+1/+0 to Wall creatures you
//! control, layer 7c) and a [`ContinuousEffect::filtered_keyword`]
//! granting [`KeywordAbility::Banding`] (layer 6).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fortified Area");
    let _wall = reg.interner_mut().intern("Wall");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
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

/// ETB trigger: install "+1/+0 to Wall creatures you control" and "Wall
/// creatures you control have banding", both anchored to this
/// enchantment and lasting until it leaves the battlefield.
fn etb_install(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wall = reg
        .interner()
        .lookup("Wall")
        .expect("Wall interned during register()");
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_subtype_sym(wall);
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_pump(
                trig.source,
                filter.clone(),
                1,
                0,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::filtered_keyword(
                trig.source,
                filter,
                KeywordAbility::Banding,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
