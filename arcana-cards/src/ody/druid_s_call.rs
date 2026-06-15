//! Druid's Call — `{1}{G}` enchantment — Aura.
//! "Enchant creature. Whenever enchanted creature is dealt damage, its
//!  controller creates that many 1/1 green Squirrel creature tokens."
//!
//! Host-trigger Aura. The grant is purely an `AttachedCreatureDoes
//! { SelfIsDealtDamage }` trigger: it reads `damage_amount()` and creates
//! that many 1/1 green Squirrel tokens for the host's controller.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Druid's Call");
    let aura = reg.interner_mut().intern("Aura");
    let _squirrel = reg.interner_mut().intern("Squirrel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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
                    condition: Box::new(TriggerCondition::SelfIsDealtDamage {
                        combat_only: false,
                    }),
                },
                intervening_if: None,
                effect: on_host_damaged,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_host_damaged(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(amount) = trig.damage_amount() else {
        return Vec::new();
    };
    if amount == 0 {
        return Vec::new();
    }
    let Some(host) = state.object_or_lki(trig.source).and_then(|o| o.attached_to) else {
        return Vec::new();
    };
    let Some(controller) = state.object_or_lki(host).map(|o| o.controller) else {
        return Vec::new();
    };
    let squirrel = reg
        .interner()
        .lookup("Squirrel")
        .expect("Squirrel interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    let token = TokenDefinition {
        name: squirrel,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    (0..amount)
        .map(|_| Effect::CreateToken {
            controller,
            token: token.clone(),
        })
        .collect()
}
