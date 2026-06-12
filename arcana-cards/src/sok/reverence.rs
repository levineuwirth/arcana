//! Reverence — `{2}{W}{W}` enchantment. "Creatures with power 2 or
//! less can't attack you."
//!
//! Implementation: ETB-installed
//! `ContinuousEffect::filtered_cant_attack` over opponent-controlled
//! creatures with power 2 or less, with
//! `Duration::WhileSourceOnBattlefield`.
//!
//! GAP (scope approximation): the printed restriction is "can't
//! attack YOU" — `filtered_cant_attack` is unconditional on the
//! defender, so matching small creatures also can't attack your
//! planeswalkers. The filter is opponent-scoped so your own small
//! creatures attack freely; in two-player play without planeswalker
//! defenders this is exact.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Reverence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
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
                effect: etb_install_attack_restriction,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "opponents' creatures with power 2 or less
/// can't attack", lasting until this enchantment leaves the
/// battlefield. See the module GAP note on the "attack you" scope.
fn etb_install_attack_restriction(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_cant_attack(
            trig.source,
            ObjectFilter::creature()
                .controlled_by(ControllerConstraint::Opponent)
                .with_max_power(2),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
