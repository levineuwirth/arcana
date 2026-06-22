//! Winged Coatl — `{1}{G}{U}` 1/1 green/blue Snake with Flash, Flying,
//! and Deathtouch. All three are evergreen keyword abilities recorded
//! as base characteristics; the runtime pipelines do the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Winged Coatl");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::Flash,
            KeywordAbility::Flying,
            KeywordAbility::Deathtouch,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
