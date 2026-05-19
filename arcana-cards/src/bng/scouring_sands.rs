//! Scouring Sands — `{1}{R}` sorcery. "Scouring Sands deals 1 damage to each creature
//! your opponents control. Scry 1."
//!
//! GAP: DealDamage to each creature opponents control requires enumerating battlefield
//! ObjectIds filtered by controller — no demonstrated GameState API for that in the catalog.
//! Scry 1 is expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scouring Sands");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Scouring Sands deals 1 damage to each creature your opponents control. Scry 1.".into(),
                target_requirements: vec![],
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
    // GAP: DealDamage to each creature opponents control — requires enumerating opponent-controlled
    // creature ObjectIds from state; no demonstrated API for that enumeration in the catalog.
    vec![
        Effect::Scry { player: entry.controller, count: 1 },
    ]
}
