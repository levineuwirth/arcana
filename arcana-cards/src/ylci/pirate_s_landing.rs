//! Pirate's Landing — `{R}` sorcery. "Draw a card. If mana from a Treasure was
//! spent to cast this spell, seek a Pirate card instead."
//!
//! GAP: Seek keyword not in usable keyword surface; conditional branch on
//! mana-source type (Treasure) is not expressible. Only the base draw is
//! emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pirate's Landing");
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
                text: "Draw a card. If mana from a Treasure was spent to cast this spell, seek a Pirate card instead.".into(),
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
    // GAP: Seek keyword / mana-source conditional not supported
    vec![Effect::DrawCards { player: entry.controller, count: 1 }]
}
