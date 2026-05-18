//! Marisi's Twinclaws — `{2}{R/W}{G}` 2/4 Cat Warrior with Double Strike.
//!
//! Red-white-green hybrid multicolor creature.
//!
//! # Rules references
//!
//! * CR 702.4 — Double strike. This creature deals both first-strike and
//!   regular combat damage.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marisi's Twinclaws");
    let cat = reg.interner_mut().intern("Cat");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(warrior);

    let colors = ColorSet::green() | ColorSet::red() | ColorSet::white();

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R/W}{G}").expect("valid cost")),
        colors,
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
