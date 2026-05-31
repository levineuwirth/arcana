//! Experimental Augury — `{1}{U}` instant. "Look at the top three
//! cards of your library. Put one of them into your hand and the rest
//! on the bottom of your library in any order. Proliferate."

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Experimental Augury");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        // Proliferate is not in the usable keyword surface and is a
        // spell-resolution effect here, not a static keyword.
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Look at the top three cards of your library. Put one of them into your hand and the rest on the bottom of your library in any order. Proliferate.".into(),
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
    // GAP: no Proliferate Effect variant — the dig portion is emitted
    // faithfully, but "Proliferate" cannot be expressed with the
    // available Effect catalog.
    vec![Effect::DigTopN {
        player: entry.controller,
        count: 3,
        filter: None,
        rest: DigRest::BottomRandom,
    }]
}
