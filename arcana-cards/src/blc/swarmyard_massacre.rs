//! Swarmyard Massacre — `{3}{B}{B}` sorcery. "Create two 1/1 green Squirrel
//! creature tokens. Then each creature that isn't an Insect, Rat, Spider,
//! or Squirrel gets -1/-1 until end of turn for each creature you control
//! that's an Insect, Rat, Spider, or Squirrel."
//!
//! GAP: variable pump/debuff based on count of creatures you control of
//! specific subtypes is not expressible with the current Effect catalog.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Swarmyard Massacre");
    let _squirrel = reg.interner_mut().intern("Squirrel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create two 1/1 green Squirrel creature tokens. Then each creature that isn't an Insect, Rat, Spider, or Squirrel gets -1/-1 until end of turn for each creature you control that's an Insect, Rat, Spider, or Squirrel.".into(),
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
    let squirrel = reg.interner().lookup("Squirrel").expect("interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    let token = TokenDefinition {
        name: squirrel,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    // GAP: variable -1/-1 debuff based on count of Insect/Rat/Spider/Squirrel you control
    vec![
        Effect::CreateToken { controller: entry.controller, token: token.clone() },
        Effect::CreateToken { controller: entry.controller, token },
    ]
}
