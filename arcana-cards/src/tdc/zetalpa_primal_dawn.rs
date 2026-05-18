//! Zetalpa, Primal Dawn — `{6}{W}{W}` 4/8 Legendary Elder Dinosaur with
//! Flying, Double Strike, Vigilance, Trample, and Indestructible.
//!
//! # Rules references
//!
//! * CR 702.9  — Flying. Can only be blocked by creatures with Flying or Reach.
//! * CR 702.4  — Double Strike. Deals both first-strike and regular combat damage.
//! * CR 702.20 — Vigilance. Attacking doesn't cause the creature to tap.
//! * CR 702.19 — Trample. Excess combat damage can be assigned to the player.
//! * CR 702.12 — Indestructible. Can't be destroyed by damage or "destroy" effects.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zetalpa, Primal Dawn");
    let elder = reg.interner_mut().intern("Elder");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elder);
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::DoubleStrike,
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
            KeywordAbility::Indestructible,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
