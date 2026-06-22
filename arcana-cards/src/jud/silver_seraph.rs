//! Silver Seraph — `{5}{W}{W}{W}` 6/6 Angel.
//! "Flying.
//!  Threshold — Other creatures you control get +2/+2 as long as there
//!  are seven or more cards in your graveyard."
//!
//! Flying is a keyword. The Threshold-gated anthem is a continuous
//! static (and Threshold is not in the supported keyword surface), so it
//! is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: keyword/static — "Threshold — Other creatures you control get +2/+2 as
// long as there are seven or more cards in your graveyard" is a graveyard-
// gated continuous anthem; Threshold is not in the supported keyword surface.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Silver Seraph");
    let angel = reg.interner_mut().intern("Angel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
