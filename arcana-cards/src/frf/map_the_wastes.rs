//! Map the Wastes — `{2}{G}` Sorcery. "Search your library for a basic
//! land card, put it onto the battlefield tapped, then shuffle. Bolster
//! 1. (Choose a creature with the least toughness among creatures you
//! control and put a +1/+1 counter on it.)"
//!
//! # Implementation note
//! TutorToBattlefield with tapped:true handles the land search.
//! Bolster 1 (choose creature with least toughness among yours, add
//! +1/+1 counter) requires a targeting decision not expressible with
//! AddCounters on a fixed target.
//!
//! # GAP
//! Bolster (choose creature with least toughness) not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Map the Wastes");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle. Bolster 1.".into(),
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
        Effect::TutorToBattlefield {
            player: entry.controller,
            filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
            tapped: true,
        },
        // GAP: Bolster 1 (choose creature with least toughness) not expressible
    ]
}
