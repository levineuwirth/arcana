//! Hermetic Study — `{1}{U}` enchantment — Aura.
//! "Enchant creature.
//!  Enchanted creature has '{T}: This creature deals 1 damage to any target.'"
//!
//! Host-activated Aura. The ETB installs an `attached_activated` continuous
//! effect granting the host '{T}: deal 1 damage to any target' (cost + effect
//! run against the host).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hermetic Study");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_activated(
            trig.source,
            ActivatedAbilityDef {
                text: "{T}: This creature deals 1 damage to any target.".into(),
                cost: ActivationCost {
                    tap: true,
                    ..Default::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: host_ping,
            },
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn host_ping(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    let target = match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => DamageTarget::Object(*id),
        Some(TargetChoice::Player(pid)) => DamageTarget::Player(*pid),
        Some(TargetChoice::ObjectOrPlayer(choice)) => match choice {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(pid) => DamageTarget::Player(*pid),
        },
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target,
        amount: 1,
    }]
}
