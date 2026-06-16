//! Ormacar, Relic Wraith — `{1}{U}{B}` 1/1 Legendary Elf Wraith with
//! Vigilance, Menace, and Lifelink.
//! "Precious (…)" — a commander-variant deckbuilding rule, GAP.
//! "As long as you control your Precious, Ormacar gets +X/+X, where X is the
//! mana value of your Precious." — dynamic continuous self-pump static, GAP.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ormacar, Relic Wraith");
    let elf = reg.interner_mut().intern("Elf");
    let wraith = reg.interner_mut().intern("Wraith");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(wraith);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Menace,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    // GAP: "Precious" deckbuilding rule and the "+X/+X where X is the mana
    // value of your Precious" continuous self-pump static are not expressible.

    reg.register(CardDefinition::new(name, chars))
}
