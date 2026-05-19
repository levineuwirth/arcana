//! Song of Blood — `{1}{R}` sorcery. "Mill four cards. Whenever a creature
//! attacks this turn, it gets +1/+0 until end of turn for each creature card
//! put into your graveyard this way."
//!
//! # GAP: delayed/until-end-of-turn triggered ability ("whenever a creature
//!   attacks this turn, +1/+0 for each creature card milled this way") not in
//!   Effect catalog

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Song of Blood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Mill four cards. Whenever a creature attacks this turn, it gets +1/+0 until end of turn for each creature card put into your graveyard this way.".into(),
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
    // GAP: delayed "attacks this turn" trigger granting +1/+0 per milled creature card
    vec![Effect::Mill { player: entry.controller, count: 4 }]
}
