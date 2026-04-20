//! Glorious Anthem — `{1}{W}` enchantment. "Creatures you control get
//! +1/+1." Canonical layer-7c anthem. First printed in Portal Second
//! Age (1998).
//!
//! Implementation: an ETB trigger adds a
//! [`ContinuousEffectKind::AnthemForController`] continuous effect
//! with duration [`Duration::WhileSourceOnBattlefield`], which the
//! layer-cleanup pipeline auto-expires when the enchantment leaves.
//!
//! Note: a "static ability" model (layer scan of all battlefield
//! permanents, constructing continuous effects each frame) would be
//! more idiomatic per CR 603 — but the layer-scan machinery doesn't
//! exist yet in `arcana-core`. ETB-trigger registration is
//! equivalent in behavior for any card that never leaves and re-
//! enters the battlefield (which is all of them in Phase 1, since
//! we don't have blink / flicker cards yet).

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
    let name = reg.interner_mut().intern("Glorious Anthem");
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
            }),
    )
}

/// ETB trigger: install "creatures you control get +1/+1" anchored
/// to this enchantment's object id, lasting until it leaves the
/// battlefield. The returned `Vec::new()` plus a state-mutating
/// effect is the Phase 2 idiom for "install a continuous effect
/// via a trigger" — the effect is applied directly to state in the
/// EffectFn since [`Effect`] doesn't currently carry a
/// `InstallContinuousEffect` variant. See the note in this module
/// for the cleaner future design.
fn etb_install_anthem(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::anthem(
            trig.source,
            trig.controller,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
