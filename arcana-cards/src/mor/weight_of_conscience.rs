//! Weight of Conscience — `{1}{W}` enchantment — Aura.
//! "Enchant creature. Enchanted creature can't attack. Tap two untapped
//!  creatures you control that share a creature type: Exile enchanted
//!  creature."
//!
//! Restriction Aura. The "can't attack" clause is an ETB-installed
//! `attached_cant_attack` marker. The activated ability whose cost is
//! "tap two untapped creatures you control that share a creature type"
//! has no expressible ActivationCost shape (tap-other-creatures with a
//! shared-type constraint is not modeled) and is GAPped.

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
    let name = reg.interner_mut().intern("Weight of Conscience");
    let aura = reg.interner_mut().intern("Aura");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(aura);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                effect: etb_install_cant_attack,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install_cant_attack(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Tap two untapped creatures that share a creature type: exile
    // enchanted creature" — tap-other-creatures shared-type cost not modeled.
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_cant_attack(
            trig.source,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}
