//! Wolf's Quarry — `{4}{G}{G}` sorcery. "Create three 1/1 green Boar
//! creature tokens with 'When this token dies, create a Food token.'"
//!
//! The Boar tokens are created; their dies-trigger (create a Food
//! token) is not modeled — token abilities are not expressible.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wolf's Quarry");
    let _boar = reg.interner_mut().intern("Boar");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create three 1/1 green Boar creature tokens with \"When this token dies, create a Food token.\"".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let boar = reg.interner().lookup("Boar").expect("interned");
    let mut sub = SubtypeSet::default();
    sub.0.insert(boar);
    let token = TokenDefinition {
        name: boar,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: sub,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: token dies-trigger "create a Food token" not modeled.
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
