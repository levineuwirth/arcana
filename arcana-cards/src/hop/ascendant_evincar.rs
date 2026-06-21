//! Ascendant Evincar — `{4}{B}{B}` 3/3 Legendary Phyrexian Vampire Noble.
//! Flying.
//! Other black creatures get +1/+1.
//! Nonblack creatures get -1/-1.
//!
//! Flying is wired. Both pump lines are pure STATIC continuous abilities
//! (no trigger word, no activation cost), which this multi-ability creature
//! shape composes only as triggered/activated abilities — so they are
//! GAP'd here (a static-only card routes to the StaticEnchantment shape).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ascendant Evincar");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let vampire = reg.interner_mut().intern("Vampire");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(vampire);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Other black creatures get +1/+1." — pure static anthem.
    // GAP: "Nonblack creatures get -1/-1." — pure static anthem.
    reg.register(CardDefinition::new(name, chars))
}
