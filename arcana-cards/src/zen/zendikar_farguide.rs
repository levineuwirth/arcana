//! Zendikar Farguide — `{4}{G}` 3/3 Elemental with Forestwalk.
//! Zendikar common; a green elemental that can't be blocked as long
//! as the defending player controls a Forest.
//!
//! # Rules references
//!
//! * CR 702.14 — Landwalk. This creature can't be blocked as long
//!   as the defending player controls a Forest.
//!   Engine wiring: `KeywordAbility::Landwalk("Forest")` is checked
//!   by the combat blocker filter against the defending player's lands.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zendikar Farguide");
    let elemental = reg.interner_mut().intern("Elemental");
    let forest = reg.interner_mut().intern("Forest");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Landwalk(forest)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
