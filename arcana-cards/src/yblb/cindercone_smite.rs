//! Cindercone Smite — `{R}` Sorcery. "Cindercone Smite deals 2 damage
//! to target creature. Then create a Treasure token if you weren't
//! the starting player."
//!
//! # Implementation note
//! DealDamage 2 to target creature is expressible. The conditional
//! Treasure token creation (if not starting player) requires a
//! "starting player" game-state check not available as a Conditional
//! condition, and Treasure tokens are not in the token catalog.
//!
//! # GAP
//! "Starting player" check not in Conditional conditions; Treasure
//! token definition not in catalog.

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
    let name = reg.interner_mut().intern("Cindercone Smite");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Cindercone Smite deals 2 damage to target creature. Then create a Treasure token if you weren't the starting player.".into(),
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
            amount: 2,
        },
        // GAP: "starting player" check not in Conditional conditions
        // GAP: Treasure token not in catalog
    ]
}
