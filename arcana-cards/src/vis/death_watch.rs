//! Death Watch — `{B}` enchantment — Aura.
//! "Enchant creature. When enchanted creature dies, its controller loses
//!  life equal to its power and you gain life equal to its toughness."
//!
//! Host death trigger (`AttachedCreatureDoes { SelfDies }`): the dying
//! creature's controller loses life equal to its power and the Aura's
//! controller gains life equal to its toughness (`script::power_of` /
//! `script::toughness_of` read the dying object's last-known P/T).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Death Watch");
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
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: |_, _, _| Vec::new(),
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

fn on_host_dies(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(dead) = trig.dying_object() else {
        return Vec::new();
    };
    let power = script::power_of(state, dead).max(0) as u32;
    let toughness = script::toughness_of(state, dead).max(0) as u32;
    let mut effects = Vec::new();
    if let Some(controller) = state.objects.get(dead).map(|o| o.controller) {
        if power > 0 {
            effects.push(Effect::LoseLife {
                player: controller,
                amount: power,
            });
        }
    }
    if let Some(you) = state.objects.get(trig.source).map(|o| o.controller) {
        if toughness > 0 {
            effects.push(Effect::GainLife {
                player: you,
                amount: toughness,
            });
        }
    }
    effects
}
