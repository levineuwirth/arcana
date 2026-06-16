//! Thrasta, Tempest's Roar — `{10}{G}{G}` 7/7 Legendary Dinosaur.
//! This spell costs {3} less to cast for each other spell cast this turn. (cost reduction — GAP)
//! Trample, haste.
//! Trample over planeswalkers. (not a supported keyword — GAP)
//! Thrasta has hexproof as long as it entered this turn. (conditional static — GAP)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thrasta, Tempest's Roar");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    // GAP: "costs {3} less for each other spell cast this turn" — no cost-
    // reduction primitive in the documented surface.
    // GAP: "Trample over planeswalkers" — not a supported KeywordAbility.
    // GAP: "has hexproof as long as it entered this turn" — conditional static.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{10}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
