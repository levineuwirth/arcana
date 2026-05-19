//! Pore Over the Pages — `{3}{U}{U}` sorcery, "Draw three cards, untap up to
//! two lands, then discard a card."
//!
//! # GAP
//! "Untap up to two lands" requires the player to choose which lands to
//! untap; there is no Effect for player-choice multi-target untap. DrawCards
//! and Discard are expressible.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pore Over the Pages");
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
                text: "Draw three cards, untap up to two lands, then discard a card.".into(),
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
    // GAP: player-choice "untap up to two lands" not in Effect catalog
    vec![
        Effect::DrawCards { player: entry.controller, count: 3 },
        Effect::Discard { player: entry.controller, count: 1, choice: DiscardChoice::ControllerChooses },
    ]
}
