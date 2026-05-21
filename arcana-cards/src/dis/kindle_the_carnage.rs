//! Kindle the Carnage — `{1}{R}{R}` sorcery. "Discard a card at
//! random. If you do, Kindle the Carnage deals damage equal to that
//! card's mana value to each creature. You may repeat this process
//! any number of times."
//!
//! GAP: dynamic discard + per-card-CMC sweep + repeat-any-number isn't
//! expressible — emit the random discard only.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kindle the Carnage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Discard a card at random. If you do, Kindle the Carnage deals damage equal to that card's mana value to each creature. You may repeat this process any number of times.".into(),
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
    // GAP: post-discard CMC-keyed creature wipe + repeat loop not modeled.
    vec![Effect::Discard {
        player: entry.controller,
        count: 1,
        choice: DiscardChoice::Random,
    }]
}
