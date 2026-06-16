//! Sinister Possession — `{B}` enchantment — Aura.
//! "Enchant creature. Whenever enchanted creature attacks or blocks, its
//!  controller loses 2 life."
//!
//! Pure host-trigger Aura. "Attacks or blocks" is two host triggers
//! (`AttachedCreatureDoes(SelfAttacks)` + `AttachedCreatureDoes(SelfBlocks)`),
//! each making the host's controller (read via `source.attached_to`) lose
//! 2 life.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Sinister Possession");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
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
                effect: on_host_combat,
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
                effect: on_host_combat,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_host_combat(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(src) = state.objects.get(trig.source) else {
        return Vec::new();
    };
    let Some(host) = src.attached_to else {
        return Vec::new();
    };
    let Some(host_obj) = state.objects.get(host) else {
        return Vec::new();
    };
    vec![Effect::LoseLife {
        player: host_obj.controller,
        amount: 2,
    }]
}
