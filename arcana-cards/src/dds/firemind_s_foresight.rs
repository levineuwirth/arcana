//! Firemind's Foresight — `{5}{U}{R}` instant. "Search your library for an instant card with mana
//! value 3, reveal it, and put it into your hand. Then repeat for mana values 2 and 1. Then shuffle."
//! Expressed as three TutorToHand effects with exact-CMC ObjectFilters.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Firemind's Foresight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for an instant card with mana value 3, reveal it, and put it into your hand. Then repeat this process for instant cards with mana values 2 and 1. Then shuffle.".into(),
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
    vec![
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter::new()
                .with_types(TypeLine::INSTANT.into())
                .with_exact_cmc(3),
            reveal: true,
        },
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter::new()
                .with_types(TypeLine::INSTANT.into())
                .with_exact_cmc(2),
            reveal: true,
        },
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter::new()
                .with_types(TypeLine::INSTANT.into())
                .with_exact_cmc(1),
            reveal: true,
        },
    ]
}
