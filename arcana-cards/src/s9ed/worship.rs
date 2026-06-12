//! Worship — `{3}{W}` enchantment. "If you control a creature, damage
//! that would reduce your life total to less than 1 reduces it to 1
//! instead."
//!
//! Implementation: an ETB trigger installs a
//! `ReplacementCondition::WouldDealDamageToSpecific` replacement on
//! the controller's player with
//! `ReplacementKind::DamageLifeFloor { floor: 1 }`; the "if you
//! control a creature" half lives on
//! `state_gate: Some(conditions::controls_a_creature)`. Lasts while
//! the enchantment is on the battlefield.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::replacement::{
    ReplacementCondition, ReplacementDuration, ReplacementEffect, ReplacementKind,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Worship");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
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
                effect: etb_install_life_floor,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "if you control a creature, damage that would
/// reduce your life total to less than 1 reduces it to 1 instead"
/// anchored to this enchantment.
fn etb_install_life_floor(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallReplacementEffect {
        effect: Box::new(ReplacementEffect {
            source: trig.source,
            id: 0,
            condition: ReplacementCondition::WouldDealDamageToSpecific {
                target: DamageTarget::Player(trig.controller),
            },
            kind: ReplacementKind::DamageLifeFloor { floor: 1 },
            is_self_replacement: false,
            duration: ReplacementDuration::WhileSourceOnBattlefield,
            state_gate: Some(conditions::controls_a_creature),
        }),
    }]
}
