//! Wilson, Refined Grizzly — `{1}{G}` 2/2 Legendary Bear Warrior.
//! Vigilance, Reach, Trample, Ward {2}.
//! "This spell can't be countered." — uncounterable static (GAP).
//! "Choose a Background" — not a usable keyword (GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wilson, Refined Grizzly");
    let bear = reg.interner_mut().intern("Bear");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bear);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: "Choose a Background" keyword has no usable variant.
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Reach,
            KeywordAbility::Trample,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    // GAP: "This spell can't be countered" is an uncounterable static.
    reg.register(CardDefinition::new(name, chars))
}
