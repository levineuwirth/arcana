//! Double Play — `{3}{G}{G}` sorcery. "Choose another player. Search your
//! library for a basic land card, put it onto the battlefield, then shuffle.
//! At the beginning of the first upkeep in your next game with that player,
//! search your library for a basic land card, put it onto the battlefield,
//! then shuffle."
//!
//! GAP: "at the beginning of the first upkeep in your next game" is a
//! cross-game persistent effect not expressible in the engine.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Double Play");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose another player. Search your library for a basic land card, put it onto the battlefield, then shuffle. At the beginning of the first upkeep in your next game with that player, search your library for a basic land card, put it onto the battlefield, then shuffle.".into(),
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
    // GAP: cross-game persistent trigger not expressible
    vec![
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: false,
        },
    ]
}
