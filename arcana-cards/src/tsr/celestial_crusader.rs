//! Celestial Crusader — `{2}{W}{W}` 2/2 Spirit with Flash and Flying.
//! "Split second"; "Other white creatures get +1/+1."
//!
//! Flash and Flying are usable keywords. Split second is not in the
//! usable keyword set (GAP). The "Other white creatures get +1/+1"
//! anthem is a static continuous ability — not a triggered or activated
//! ability and not expressible in this shape (GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Celestial Crusader");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Split second not in usable KeywordAbility set.
        keywords: vec![KeywordAbility::Flash, KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: "Other white creatures get +1/+1" — static anthem, not a
    // triggered/activated ability; not expressible in this shape.
    reg.register(CardDefinition::new(name, chars))
}
