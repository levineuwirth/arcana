//! Visions of Brutality — `{1}{B}` enchantment — Aura (Devoid, colorless).
//! "Devoid. Enchant creature. Enchanted creature can't block. Whenever
//!  enchanted creature deals damage, its controller loses that much life."
//!
//! Devoid makes the card colorless. "Can't block" is an ETB-installed
//! `attached_cant_block`. The "whenever enchanted creature deals damage,
//! its controller loses that much life" payoff keys on dealing damage,
//! which is not a wrappable Self* condition — GAP.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Visions of Brutality");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
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
    // GAP: "whenever enchanted creature deals damage, its controller loses
    // that much life" — "deals damage" is not a wrappable Self* condition.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_cant_block(
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
