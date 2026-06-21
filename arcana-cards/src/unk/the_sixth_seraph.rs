//! The Sixth Seraph — `{3}{W}{W}` 3/6 Legendary Angel. Flying.
//! "Artifact spells you cast have demonstrate" is a static spell-granting
//! ability (demonstrate cast-copy mechanic) with no primitive in the
//! demonstrated API — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Sixth Seraph");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: static "Artifact spells you cast have demonstrate" — no
    // spell-ability-granting / demonstrate primitive in the demonstrated API.
    reg.register(CardDefinition::new(name, chars))
}
