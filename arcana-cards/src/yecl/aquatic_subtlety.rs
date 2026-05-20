//! Aquatic Subtlety — `{U}{U}{U}` sorcery. "Draw two cards, then
//! discard two cards. Blue creature cards in your hand perpetually
//! gain 'Evoke—Exile a blue card from your hand.'"
//!
//! GAP: perpetual hand-modification ('blue creature cards in your
//! hand perpetually gain Evoke …') has no catalog Effect. Only the
//! loot half is emitted.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Aquatic Subtlety");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw two cards, then discard two cards. Blue creature cards in your hand perpetually gain \"Evoke—Exile a blue card from your hand.\"".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: perpetual "Evoke—Exile a blue card" grant on cards in hand.
    vec![
        Effect::DrawCards {
            player: entry.controller,
            count: 2,
        },
        Effect::Discard {
            player: entry.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
