//! Rapturous Moment — `{4}{U}{R}` sorcery. "Draw three cards, then
//! discard two cards. Add {U}{U}{R}{R}{R}."
//!
//! "Add mana to your pool" has no catalog Effect (spells don't have
//! a mana-production primitive); GAP'd. Draw/discard are modeled.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rapturous Moment");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw three cards, then discard two cards. Add {U}{U}{R}{R}{R}.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "add {U}{U}{R}{R}{R}" mana production not in catalog.
    vec![
        Effect::DrawCards { player: entry.controller, count: 3 },
        Effect::Discard {
            player: entry.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
