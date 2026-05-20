//! Vampiric Tutor — `{B}` instant. "Search your library for a card,
//! then shuffle and put that card on top. You lose 2 life." The
//! search-to-top-of-library mode is not expressible (only
//! TutorToHand / TutorToBattlefield exist), so only the life loss is
//! emitted as a partial.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vampiric Tutor");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Search your library for a card, then shuffle and put that card on top. You lose 2 life.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "search library for a card, shuffle, put it on top" — only
    // TutorToHand / TutorToBattlefield exist; no search-to-top-of-
    // library tutor primitive. The life loss is still applied.
    vec![Effect::LoseLife { player: entry.controller, amount: 2 }]
}
