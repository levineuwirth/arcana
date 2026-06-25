//! Impending Doom — `{2}{R}` enchantment — Aura.
//! "Enchant creature. Enchanted creature gets +3/+3 and attacks each
//!  combat if able. When enchanted creature dies, this Aura deals 3
//!  damage to that creature's controller."
//!
//! Buff + host-death-trigger Aura. ETB installs `attached_pt(+3, +3)` and a
//! `must_attack` requirement on the enchanted creature (the host id read from
//! `source.attached_to` at resolution, since the engine has already attached
//! the Aura). The death trigger (`AttachedCreatureDoes { SelfDies }`) deals 3
//! damage to the dying creature's controller (read via LKI on the dying object).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Impending Doom");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                effect: etb_install_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfDies),
                },
                intervening_if: None,
                effect: on_host_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_pump(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            3,
            3,
            Duration::WhileSourceOnBattlefield,
        ),
    }];
    // "Enchanted creature ... attacks each combat if able." The host is
    // already attached by resolution; force it to attack while this Aura
    // remains on the battlefield.
    if let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) {
        effects.push(Effect::InstallContinuousEffect {
            effect: ContinuousEffect::must_attack(
                trig.source,
                host,
                Duration::WhileSourceOnBattlefield,
            ),
        });
    }
    effects
}

fn on_host_dies(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(dead) = trig.dying_object() else {
        return Vec::new();
    };
    let Some(controller) = state.object_or_lki(dead).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(controller),
        amount: 3,
    }]
}
