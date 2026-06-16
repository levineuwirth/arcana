//! Spiteful Shadows — `{1}{B}` enchantment — Aura.
//! "Enchant creature. Whenever enchanted creature is dealt damage, it
//!  deals that much damage to its controller."
//!
//! Host damage trigger: `AttachedCreatureDoes { SelfIsDealtDamage }`
//! reads the damage dealt (`trig.damage_amount()`) and the enchanted
//! creature's controller (via `source.attached_to`), then deals that much
//! damage to that player. Source of the damage is the host creature.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
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
    let name = reg.interner_mut().intern("Spiteful Shadows");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
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
                    condition: Box::new(TriggerCondition::SelfIsDealtDamage {
                        combat_only: false,
                    }),
                },
                intervening_if: None,
                effect: on_host_dealt_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_host_dealt_damage(
    state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(amount) = trig.damage_amount() else {
        return Vec::new();
    };
    if amount == 0 {
        return Vec::new();
    }
    let Some(host) = state.objects.get(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    let Some(controller) = state.objects.get(host).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: host,
        target: DamageTarget::Player(controller),
        amount,
    }]
}
