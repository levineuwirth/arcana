//! Wolf's Quarry — `{4}{G}{G}` sorcery. "Create three 1/1 green Boar creature
//! tokens with 'When this token dies, create a Food token.'"
//!
//! GAP: triggered ability on tokens ("when this dies, create Food token") not
//! expressible in TokenDefinition.abilities (no triggered ability API there);
//! Food token not supported. Partial: three Boar tokens created without
//! death trigger.

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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create three 1/1 green Boar creature tokens with 'When this token dies, create a Food token.'".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: death-trigger "create Food token" not expressible in TokenDefinition.abilities
    // GAP: Food token not supported
    let boar = reg.interner().lookup("Boar").expect("Boar interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);
    let token = TokenDefinition {
        name: boar,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
