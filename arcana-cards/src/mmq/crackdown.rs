//! Crackdown — `{2}{W}` enchantment. "Nonwhite creatures with power 3
//! or greater don't untap during their controllers' untap steps."
//!
//! Implementation: an ETB trigger installs a
//! [`ContinuousEffect::filtered_dont_untap`] scoped to nonwhite
//! creatures with power 3 or greater, with duration
//! [`Duration::WhileSourceOnBattlefield`]; the layer-cleanup pipeline
//! auto-expires it when the enchantment leaves.

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
    let name = reg.interner_mut().intern("Crackdown");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
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
                effect: etb_install_untap_restriction,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "nonwhite creatures with power 3 or greater
/// don't untap during their controllers' untap steps", anchored to
/// this enchantment.
fn etb_install_untap_restriction(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_dont_untap(
            trig.source,
            ObjectFilter::creature()
                .without_colors(ColorSet::white())
                .with_min_power(3),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
