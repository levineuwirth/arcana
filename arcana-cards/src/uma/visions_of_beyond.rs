//! Visions of Beyond — `{U}` instant.
//! "Draw a card. If a graveyard has twenty or more cards in it, draw
//! three cards instead."
//
// GAP: conditional draw based on "a graveyard has 20+ cards" (runtime
//      graveyard size check) not expressible. Best effort: draw 1 card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Visions of Beyond");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card. If a graveyard has twenty or more cards in it, draw three cards instead.".into(),
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
    // GAP: conditional 3-card draw if any graveyard has ≥20 cards
    vec![Effect::DrawCards { player: entry.controller, count: 1 }]
}
