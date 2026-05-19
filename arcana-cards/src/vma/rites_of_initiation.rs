//! Rites of Initiation — `{R}` instant, "Discard any number of cards at random.
//! Creatures you control get +1/+0 until end of turn for each card discarded this way."
//!
//! GAP: variable pump (+N/+0 where N = number of cards discarded) requires knowing
//! how many cards were discarded, which is a runtime quantity; ForEach pump on all
//! controlled creatures scaled by discard count is not expressible with catalog effects.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rites of Initiation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Discard any number of cards at random. Creatures you control get +1/+0 until end of turn for each card discarded this way.".into(),
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
    // GAP: discard any number (player-chosen count), pump all controlled creatures scaled by discard count
    vec![Effect::Discard {
        player: entry.controller,
        count: 1,
        choice: DiscardChoice::Random,
    }]
}
