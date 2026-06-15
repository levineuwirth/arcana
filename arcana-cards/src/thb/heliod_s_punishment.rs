//! Heliod's Punishment — `{1}{W}` enchantment — Aura (Theros Beyond Death).
//! "Enchant creature. This Aura enters with four task counters on it.
//!  Enchanted creature can't attack or block. It loses all abilities and
//!  has '{T}: Remove a task counter from Heliod's Punishment. Then if it
//!  has no task counters on it, destroy Heliod's Punishment.'"
//!
//! The can't-attack / can't-block restriction is expressible as the
//! Pacifism pair. The rest is not: "enters with N task counters" (no
//! enters-with-counters on an Aura via this API), "loses all abilities"
//! (no attached_loses_all_abilities builder), and the self-referential
//! task-counter activated ability — all GAP'd.

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
    let name = reg.interner_mut().intern("Heliod's Punishment");
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
                effect: etb_install,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_install(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    // GAP: "enters with four task counters" (no enters-with-counters on Aura),
    // "loses all abilities" (no attached_loses_all_abilities builder), and the
    // self-referential "{T}: remove a task counter; destroy if none" ability.
    // The can't-attack / can't-block restriction IS expressed below.
    vec![
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_cant_attack(
                trig.source,
                Duration::WhileSourceOnBattlefield,
            ),
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::attached_cant_block(
                trig.source,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
