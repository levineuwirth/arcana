//! Cruel Tutor — `{2}{B}` sorcery, "Search your library for a card, then
//! shuffle and put that card on top. You lose 2 life."
//!
//! GAP: TutorToTopOfLibrary (search then put on top) is not in the effect
//! catalog. Best-effort: TutorToHand (closest available search effect) plus
//! LoseLife for the 2 life payment.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cruel Tutor");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
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
    // GAP: no TutorToTopOfLibrary effect variant; using TutorToHand as best approximation
    vec![
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter::new(),
            reveal: false,
        },
        Effect::LoseLife { player: entry.controller, amount: 2 },
    ]
}
