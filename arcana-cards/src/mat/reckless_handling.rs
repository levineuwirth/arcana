//! Reckless Handling — `{1}{R}` sorcery. "Search your library for an artifact card, reveal it, put it
//! into your hand, shuffle, then discard a card at random. If an artifact card was discarded this way,
//! Reckless Handling deals 2 damage to each opponent."
//! GAP: "If an artifact card was discarded this way" conditional — Effect::Conditional requires a
//! condition type not available for checking the type of the randomly discarded card.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Reckless Handling");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for an artifact card, reveal it, put it into your hand, shuffle, then discard a card at random. If an artifact card was discarded this way, Reckless Handling deals 2 damage to each opponent.".into(),
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
    // GAP: "if an artifact card was discarded this way, deal 2 damage to each opponent" —
    // no condition variant for checking the type of a randomly discarded card
    vec![
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
            reveal: true,
        },
        Effect::Discard { player: entry.controller, count: 1, choice: DiscardChoice::Random },
    ]
}
