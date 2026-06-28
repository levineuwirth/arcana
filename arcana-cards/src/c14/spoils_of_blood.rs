//! Spoils of Blood — `{B}` instant. "Create an X/X black Horror creature
//! token, where X is the number of creatures that died this turn."
//! Dynamic X via script::creatures_died_this_turn.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spoils of Blood");
    let _horror = reg.interner_mut().intern("Horror");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create an X/X black Horror creature token, where X is the number of creatures that died this turn.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::creatures_died_this_turn(state);
    let horror = reg
        .interner()
        .lookup("Horror")
        .expect("Horror interned during register()");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horror);
    let token = TokenDefinition {
        name: horror,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(x as i32)),
        toughness: Some(PtValue::Fixed(x as i32)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: entry.controller, token }]
}
