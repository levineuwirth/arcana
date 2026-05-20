//! Bestial Menace — `{3}{G}{G}` sorcery. "Create a 1/1 green Snake
//! creature token, a 2/2 green Wolf creature token, and a 3/3 green
//! Elephant creature token."

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bestial Menace");
    let _snake = reg.interner_mut().intern("Snake");
    let _wolf = reg.interner_mut().intern("Wolf");
    let _elephant = reg.interner_mut().intern("Elephant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a 1/1 green Snake creature token, a 2/2 green Wolf creature token, and a 3/3 green Elephant creature token.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn token(reg: &CardRegistry, subtype: &str, p: i32) -> TokenDefinition {
    let s = reg.interner().lookup(subtype).expect("subtype interned");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(s);
    TokenDefinition {
        name: s,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(p)),
        toughness: Some(PtValue::Fixed(p)),
        keywords: vec![],
        abilities: vec![],
    }
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::CreateToken {
            controller: entry.controller,
            token: token(reg, "Snake", 1),
        },
        Effect::CreateToken {
            controller: entry.controller,
            token: token(reg, "Wolf", 2),
        },
        Effect::CreateToken {
            controller: entry.controller,
            token: token(reg, "Elephant", 3),
        },
    ]
}
