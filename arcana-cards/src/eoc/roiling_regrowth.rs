//! Roiling Regrowth — `{2}{G}` instant. "Sacrifice a land. Search your library
//! for up to two basic land cards, put them onto the battlefield tapped, then
//! shuffle."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roiling Regrowth");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Sacrifice a land. Search your library for up to two basic land cards, put them onto the battlefield tapped, then shuffle.".into(),
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
    let land = ObjectFilter::new().with_types(TypeLine::LAND.into());
    vec![
        Effect::Sacrifice {
            player: entry.controller,
            filter: land.clone(),
            count: 1,
        },
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: land.clone(),
            tapped: true,
        },
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: land,
            tapped: true,
        },
    ]
}
