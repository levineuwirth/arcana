//! Bestial Menace — `{3}{G}{G}` sorcery, "Create a 1/1 green Snake
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 1/1 green Snake creature token, a 2/2 green Wolf creature token, and a 3/3 green Elephant creature token.".into(),
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
    let snake = reg.interner().lookup("Snake").expect("Snake interned");
    let wolf = reg.interner().lookup("Wolf").expect("Wolf interned");
    let elephant = reg.interner().lookup("Elephant").expect("Elephant interned");

    let mut snake_subtypes = SubtypeSet::default();
    snake_subtypes.0.insert(snake);
    let mut wolf_subtypes = SubtypeSet::default();
    wolf_subtypes.0.insert(wolf);
    let mut elephant_subtypes = SubtypeSet::default();
    elephant_subtypes.0.insert(elephant);

    vec![
        Effect::CreateToken {
            controller: entry.controller,
            token: TokenDefinition {
                name: snake,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: snake_subtypes,
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        },
        Effect::CreateToken {
            controller: entry.controller,
            token: TokenDefinition {
                name: wolf,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: wolf_subtypes,
                power: Some(PtValue::Fixed(2)),
                toughness: Some(PtValue::Fixed(2)),
                keywords: vec![],
                abilities: vec![],
            },
        },
        Effect::CreateToken {
            controller: entry.controller,
            token: TokenDefinition {
                name: elephant,
                colors: ColorSet::green(),
                types: TypeLine::CREATURE.into(),
                subtypes: elephant_subtypes,
                power: Some(PtValue::Fixed(3)),
                toughness: Some(PtValue::Fixed(3)),
                keywords: vec![],
                abilities: vec![],
            },
        },
    ]
}
