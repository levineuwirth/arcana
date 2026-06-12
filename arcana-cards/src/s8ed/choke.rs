//! Choke — `{2}{G}` enchantment. "Islands don't untap during their
//! controllers' untap steps."
//!
//! Implementation: an ETB trigger installs a
//! [`ContinuousEffect::filtered_dont_untap`] scoped to lands with the
//! Island subtype, with duration
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
    let name = reg.interner_mut().intern("Choke");
    // Intern "Island" now so the effect fn's lookup is guaranteed to hit.
    let _island = reg.interner_mut().intern("Island");
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
                effect: etb_install_untap_restriction,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// ETB trigger: install "Islands don't untap during their
/// controllers' untap steps", anchored to this enchantment.
fn etb_install_untap_restriction(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let island = reg
        .interner()
        .lookup("Island")
        .expect("Island interned during register()");
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::filtered_dont_untap(
            trig.source,
            ObjectFilter {
                types: Some(TypeLine::LAND.into()),
                ..Default::default()
            }
            .with_subtype_sym(island),
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
