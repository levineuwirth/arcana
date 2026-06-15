//! Binding Agony — `{1}{B}` enchantment — Aura.
//! "Enchant creature. Whenever enchanted creature is dealt damage, this Aura
//!  deals that much damage to that creature's controller."
//!
//! Host-trigger Aura. The grant is purely a host trigger, so the only
//! ability is an `AttachedCreatureDoes { SelfIsDealtDamage }` trigger that
//! reflects `damage_amount()` back at the host creature's controller. No
//! ETB pump/keyword install is needed.

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
    let name = reg.interner_mut().intern("Binding Agony");
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
                id: 2,
                trigger_condition: TriggerCondition::AttachedCreatureDoes {
                    condition: Box::new(TriggerCondition::SelfIsDealtDamage {
                        combat_only: false,
                    }),
                },
                intervening_if: None,
                effect: reflect_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn reflect_damage(state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let Some(amount) = trig.damage_amount() else {
        return Vec::new();
    };
    if amount == 0 {
        return Vec::new();
    }
    // The host creature (this Aura's enchanted creature) is what was dealt the
    // damage; deal that much to its controller.
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    let Some(controller) = state.object_or_lki(host).map(|o| o.controller) else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(controller),
        amount,
    }]
}
