//! Tend the Sprigs — `{2}{G}` sorcery. "Search your library for a basic land card, put it onto
//! the battlefield tapped, then shuffle. If you control seven or more lands or a Treefolk,
//! instead put that land onto the battlefield untapped."
//!
//! # GAP
//! - No support for conditional tapped/untapped land entry based on controlling 7+ lands
//!   or a Treefolk (subtype filter)
//! - TutorToBattlefield does not support a conditional tapped parameter

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tend the Sprigs");
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
                text: "Search your library for a basic land card, put it onto the battlefield tapped, then shuffle. If you control seven or more lands or a Treefolk, instead put that land onto the battlefield untapped.".into(),
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
    // GAP: conditional tapped/untapped based on 7+ lands or controlling a Treefolk
    vec![Effect::TutorToBattlefield {
        player: entry.controller,
        filter: ObjectFilter::new().with_types(TypeLine::LAND.into()),
        tapped: true,
    }]
}
