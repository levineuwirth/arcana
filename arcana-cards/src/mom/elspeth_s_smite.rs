//! Elspeth's Smite — `{W}` instant.
//! "Elspeth's Smite deals 3 damage to target attacking or blocking creature.
//! If that creature would die this turn, exile it instead."
//!
//! GAP: no TargetFilter for "attacking or blocking creature"; using
//! target_creature() as best-effort approximation.
//! GAP: replacement effect "if it would die, exile it instead" not in catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elspeth's Smite");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Elspeth's Smite deals 3 damage to target attacking or blocking creature. If that creature would die this turn, exile it instead.".into(),
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
            target: arcana_core::events::DamageTarget::Object(*id),
            amount: 3,
        },
        // GAP: replacement effect "if it would die this turn, exile it instead" not in catalog.
    ]
}
