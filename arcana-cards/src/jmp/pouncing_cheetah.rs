//! Pouncing Cheetah — `{2}{G}` 3/2 Cat with Flash.
//! Hour of Devastation common; a green cat that can be cast at instant
//! speed to ambush attackers or slip in as a surprise blocker.
//!
//! # Rules references
//!
//! * CR 702.8 — Flash. "You may cast this spell any time you could cast
//!   an instant." Engine checks this during the casting window.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pouncing Cheetah");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flash],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
