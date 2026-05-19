//! Words of Wisdom — `{1}{U}` instant.
//! "You draw two cards, then each other player draws a card."
//!
//! # GAP: EachOtherPlayerDraws — Effect::DrawCards targets a single PlayerId; there is no
//! Effect variant for applying a draw to each player other than the controller. Best effort:
//! draw 2 for the controller; the "each other player draws a card" clause is dropped.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Words of Wisdom");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You draw two cards, then each other player draws a card.".into(),
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
    // GAP: EachOtherPlayerDraws — no Effect variant for drawing cards for each player other
    // than the controller.
    vec![Effect::DrawCards { player: entry.controller, count: 2 }]
}
