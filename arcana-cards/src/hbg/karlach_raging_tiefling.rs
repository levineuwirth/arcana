//! Karlach, Raging Tiefling — `{1}{R}` 2/2 Legendary Tiefling Barbarian.
//! First strike.
//! Rage Beyond Death — Specialize {6}. You may also activate this ability if
//! Karlach is in your graveyard.
//!
//! Specialize is not an expressible activation cost (no Specialize cost field
//! in ActivationCost; the colored-back transform is unmodeled) and "Rage
//! Beyond Death" is not a usable KeywordAbility. GAP'd; First strike retained.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Karlach, Raging Tiefling");
    let tiefling = reg.interner_mut().intern("Tiefling");
    let barbarian = reg.interner_mut().intern("Barbarian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(tiefling);
    subtypes.0.insert(barbarian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: "Rage Beyond Death — Specialize {6}" — Specialize is not an
    //      expressible activation cost; graveyard-activation rider unexpressible.
    reg.register(CardDefinition::new(name, chars))
}
