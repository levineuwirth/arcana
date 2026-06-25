//! Nettling Curse — `{2}{B}` enchantment — Aura.
//! "Enchant creature. Whenever enchanted creature attacks or blocks, its
//!  controller loses 3 life. {1}{R}: Enchanted creature attacks this turn
//!  if able."
//!
//! The "attacks or blocks" payoff is wired as two host triggers
//! (AttachedCreatureDoes wrapping SelfAttacks and SelfBlocks); each makes
//! the enchanted creature's controller lose 3 life. The "{1}{R}: Enchanted
//! creature attacks this turn if able" host-activated ability installs an
//! EndOfTurn must-attack continuous effect on the enchanted creature.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nettling Curse");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enchant(TargetFilter::Creature)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfAttacks),
                },
                intervening_if: None,
                effect: host_controller_loses_3,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfBlocks),
                },
                intervening_if: None,
                effect: host_controller_loses_3,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: Enchanted creature attacks this turn if able.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: enchanted_must_attack,
            }),
    )
}

fn enchanted_must_attack(
    state: &GameState,
    ctx: &ActivationContext,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(host) = state.objects.get(ctx.source).and_then(|o| o.attached_to)
    else {
        return Vec::new();
    };
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::must_attack(ctx.source, host, Duration::EndOfTurn),
    }]
}

fn host_controller_loses_3(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(host) = state.objects.get(trig.source).and_then(|o| o.attached_to)
    else {
        return Vec::new();
    };
    let Some(controller) = state.objects.get(host).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::LoseLife { player: controller, amount: 3 }]
}
