//! Dan Lewis — `{1}{R}` 2/2 Legendary Human. Static: noncreature, non-Equipment
//! artifacts you control become Equipment with "Equipped creature gets +1/+0"
//! and equip {1}. Doctor's companion.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dan Lewis");
    let human = reg.interner_mut().intern("Human");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static "noncreature, non-Equipment artifacts you control are
    // Equipment ... and have '+1/+0' / equip {1}" — board-wide type/ability
    // grant, no expressible primitive.
    // GAP: "Doctor's companion" — not a usable KeywordAbility variant.
    reg.register(CardDefinition::new(name, chars))
}
