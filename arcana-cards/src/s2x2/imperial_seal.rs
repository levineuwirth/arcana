//! Imperial Seal — `{B}` sorcery.
//! "Search your library for a card, then shuffle and put that card on top. You lose 2 life."
//! GAP: TutorToHand / TutorToBattlefield find but put on top of library; no TutorToTopOfLibrary
//! variant in the catalog — using Vec::new() for the tutor half.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Imperial Seal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a card, then shuffle and put that card on top. You lose 2 life.".into(),
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
    // GAP: no TutorToTopOfLibrary (any-card tutor that puts result on top rather than into hand/battlefield)
    vec![
        Effect::LoseLife { player: entry.controller, amount: 2 },
    ]
}
