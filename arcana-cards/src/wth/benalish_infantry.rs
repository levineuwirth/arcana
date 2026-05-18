//! Benalish Infantry — `{2}{W}` 1/3 Human Soldier with Banding.
//! Mirage common; a defensive white creature representing the stalwart
//! Benalish soldiery through the banding keyword.
//!
//! # Rules references
//!
//! * CR 702.22 — Banding. Creatures with banding can attack or block as a band,
//!   with the controller of creatures with banding in the band assigning combat
//!   damage for any creature blocked by or blocking the band.
//!
//! Banding is the sole keyword; listing it in `keywords` is sufficient —
//! the runtime pipelines do the rest.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Benalish Infantry");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Banding],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
