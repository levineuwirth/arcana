//! Sound the Call — `{2}{G}` sorcery. "Create a 1/1 green Wolf creature
//! token. It has 'This token gets +1/+1 for each card named Sound the
//! Call in each graveyard.'"
//!
//! The Wolf token is created faithfully (1/1 green Wolf). Its granted
//! static self-buff ("+1/+1 for each card named Sound the Call in each
//! graveyard") is a dynamic continuous ability on the token itself; the
//! `TokenDefinition.abilities` surface here cannot express a
//! graveyard-counting power/toughness boost, so that rider is a GAP.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sound the Call");
    let _wolf = reg.interner_mut().intern("Wolf");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a 1/1 green Wolf creature token. It has \"This token gets +1/+1 for each card named Sound the Call in each graveyard.\"".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let wolf = reg
        .interner()
        .lookup("Wolf")
        .expect("Wolf interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wolf);
    let token = TokenDefinition {
        name: wolf,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        // GAP: the token's granted dynamic static ability ("gets +1/+1 for
        // each card named Sound the Call in each graveyard") cannot be
        // expressed via TokenDefinition.abilities.
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: entry.controller,
        token,
    }]
}
