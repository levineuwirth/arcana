//! Heed the Mists — `{3}{U}{U}` sorcery — Arcane. "Mill a card, then
//! draw cards equal to the milled card's mana value." We can't
//! inspect the just-milled card with the catalog helpers — best
//! effort: mill 1 and draw 1.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Heed the Mists");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Mill a card, then draw cards equal to the milled card's mana value.".into(),
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
    // GAP: 'draws equal to the milled card's mana value' — no helper to
    // inspect the top-of-library card before/after milling for its CMC.
    vec![
        Effect::Mill { player: entry.controller, count: 1 },
        Effect::DrawCards { player: entry.controller, count: 1 },
    ]
}
