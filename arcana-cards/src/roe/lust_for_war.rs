//! Lust for War — `{2}{R}` enchantment — Aura.
//! "Enchant creature. Whenever enchanted creature becomes tapped, this Aura
//!  deals 3 damage to that creature's controller. Enchanted creature attacks
//!  each combat if able."
//!
//! The host "becomes tapped" trigger is an `AttachedCreatureDoes`
//! (`SelfBecomesTapped`) that deals 3 damage to the host's controller (read
//! via `source.attached_to`, the Gremlin Infestation exemplar). "Enchanted
//! creature attacks each combat if able" installs a `must_attack` requirement
//! on the host (CR 508.1a) at ETB, the host read from the Aura's `attached_to`.

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
    let name = reg.interner_mut().intern("Lust for War");
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
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfBecomesTapped),
                },
                intervening_if: None,
                effect: tapped_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // "Enchanted creature attacks each combat if able" — the Aura entered
    // already attached, so the host is its current `attached_to`.
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(
            trig.source,
            host,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn tapped_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    let Some(host_obj) = state.objects.get(host) else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(host_obj.controller),
        amount: 3,
    }]
}
