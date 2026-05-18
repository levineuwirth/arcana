//! Kjeldoran Warrior — `{W}` 1/1 Human Warrior with Banding.
//! Ice Age common; a cheap white creature representing the disciplined
//! Kjeldoran military tradition through the banding keyword.
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
    let name = reg.interner_mut().intern("Kjeldoran Warrior");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Banding],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
