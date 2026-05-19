//! Conflux — `{3}{W}{U}{B}{R}{G}` sorcery.
//! "Search your library for a white card, a blue card, a black card, a red
//! card, and a green card. Reveal those cards, put them into your hand, then shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conflux");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a white card, a blue card, a black card, a red card, and a green card. Reveal those cards, put them into your hand, then shuffle.".into(),
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
            filter: ObjectFilter::new().with_colors(ColorSet::white()),
            reveal: true,
        },
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter::new().with_colors(ColorSet::blue()),
            reveal: true,
        },
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter::new().with_colors(ColorSet::black()),
            reveal: true,
        },
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter::new().with_colors(ColorSet::red()),
            reveal: true,
        },
        Effect::TutorToHand {
            player: entry.controller,
            filter: ObjectFilter::new().with_colors(ColorSet::green()),
            reveal: true,
        },
    ]
}
