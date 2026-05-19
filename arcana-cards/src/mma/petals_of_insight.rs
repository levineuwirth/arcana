//! Petals of Insight — `{4}{U}` Sorcery — Arcane. "Look at the top three
//! cards of your library. You may put those cards on the bottom of your
//! library in any order. If you do, return Petals of Insight to its owner's
//! hand. Otherwise, draw three cards."
//!
//! GAP: conditional logic based on player choice (put-on-bottom → return-to-
//! hand vs draw-3) is not expressible with the available Effect catalog.
//! Best effort: draw 3 (the non-return branch).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Petals of Insight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Look at the top three cards of your library. You may put those cards on the bottom of your library in any order. If you do, return Petals of Insight to its owner's hand. Otherwise, draw three cards.".into(),
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
    // GAP: look at top 3 with player-choice branch (bottom+return or draw 3)
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
