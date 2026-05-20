//! Mascot Exhibition — `{7}` sorcery. "Create a 2/1 white and black
//! Inkling creature token with flying, a 3/2 red and white Spirit
//! creature token, and a 4/4 blue and red Elemental creature token."

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mascot Exhibition");
    let _inkling = reg.interner_mut().intern("Inkling");
    let _spirit = reg.interner_mut().intern("Spirit");
    let _elemental = reg.interner_mut().intern("Elemental");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Create a 2/1 white and black Inkling creature token with flying, a 3/2 red and white Spirit creature token, and a 4/4 blue and red Elemental creature token.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let inkling = reg.interner().lookup("Inkling").expect("interned");
    let spirit = reg.interner().lookup("Spirit").expect("interned");
    let elemental = reg.interner().lookup("Elemental").expect("interned");

    let mut ink_sub = SubtypeSet::default();
    ink_sub.0.insert(inkling);
    let inkling_tok = TokenDefinition {
        name: inkling,
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: ink_sub,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    };

    let mut spi_sub = SubtypeSet::default();
    spi_sub.0.insert(spirit);
    let spirit_tok = TokenDefinition {
        name: spirit,
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: spi_sub,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };

    let mut ele_sub = SubtypeSet::default();
    ele_sub.0.insert(elemental);
    let elemental_tok = TokenDefinition {
        name: elemental,
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: ele_sub,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
    };

    vec![
        Effect::CreateToken { controller: entry.controller, token: inkling_tok },
        Effect::CreateToken { controller: entry.controller, token: spirit_tok },
        Effect::CreateToken { controller: entry.controller, token: elemental_tok },
    ]
}
