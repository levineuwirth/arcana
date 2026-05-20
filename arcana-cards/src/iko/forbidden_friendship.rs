//! Forbidden Friendship — `{1}{R}` sorcery. "Create a 1/1 red Dinosaur
//! creature token with haste and a 1/1 white Human Soldier creature
//! token."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forbidden Friendship");
    let _dino = reg.interner_mut().intern("Dinosaur");
    let _human = reg.interner_mut().intern("Human");
    let _soldier = reg.interner_mut().intern("Soldier");
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
                text: "Create a 1/1 red Dinosaur creature token with haste and a 1/1 white Human Soldier creature token.".into(),
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
    let dino = reg.interner().lookup("Dinosaur").expect("Dinosaur interned");
    let human = reg.interner().lookup("Human").expect("Human interned");
    let soldier = reg.interner().lookup("Soldier").expect("Soldier interned");

    let mut dino_subs = SubtypeSet::default();
    dino_subs.0.insert(dino);
    let dino_token = TokenDefinition {
        name: dino,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: dino_subs,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        abilities: vec![],
    };

    let mut human_subs = SubtypeSet::default();
    human_subs.0.insert(human);
    human_subs.0.insert(soldier);
    let human_token = TokenDefinition {
        name: human,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: human_subs,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };

    vec![
        Effect::CreateToken { controller: entry.controller, token: dino_token },
        Effect::CreateToken { controller: entry.controller, token: human_token },
    ]
}
