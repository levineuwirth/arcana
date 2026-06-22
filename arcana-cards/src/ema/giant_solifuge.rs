//! Giant Solifuge — `{2}{R/G}{R/G}` 4/1 Insect with Trample, Haste,
//! and Shroud. ({R/G} can be paid with either {R} or {G}.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Giant Solifuge");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R/G}{R/G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Haste,
            KeywordAbility::Shroud,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
