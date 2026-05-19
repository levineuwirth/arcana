//! Overwhelming Victory — `{4}{R}` Instant. "Overwhelming Victory
//! deals 5 damage to target creature. Each creature you control gets
//! +X/+0 until end of turn, where X is the excess damage dealt this
//! way."
//!
//! # Implementation note
//! The 5 damage to target creature is expressible. The pump on all
//! your creatures based on excess damage is not expressible (no excess
//! damage tracking in Effect catalog).
//!
//! # GAP
//! Excess damage calculation and applying it as a pump effect not
//! expressible.

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
    let name = reg.interner_mut().intern("Overwhelming Victory");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Overwhelming Victory deals 5 damage to target creature. Each creature you control gets +X/+0 until end of turn, where X is the excess damage dealt this way.".into(),
                target_requirements: vec![TargetRequirement::target_creature()],
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::DealDamage {
            source: entry.source,
            target: DamageTarget::Object(*id),
            amount: 5,
        },
        // GAP: excess damage pump on your creatures not expressible
    ]
}
