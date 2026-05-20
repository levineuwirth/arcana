//! Lunge — `{2}{R}` instant. "Lunge deals 2 damage to target
//! creature and 2 damage to target player or planeswalker."
//!
//! Two targets: a creature and a player. (Planeswalker damage is
//! folded into the player target — no separate planeswalker target
//! filter is needed for the player branch.)

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lunge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Lunge deals 2 damage to target creature and 2 damage to target player or planeswalker.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_player(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(creature)) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(TargetChoice::Player(p)) = entry.targets.targets.get(1) else { return Vec::new(); };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*creature),
            amount: 2,
        },
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Player(*p),
            amount: 2,
        },
    ]
}
