//! Sun-Dappled Celebrant — `{4}{W}{W}` 5/6 Creature — Treefolk Cleric.
//! Convoke.
//! Vigilance.
//!
//! Vigilance is a base keyword. Convoke is not in the usable
//! `KeywordAbility` surface for this card class (the alternative-cost
//! tap-creatures-to-help-cast mechanic is not modeled here) — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sun-Dappled Celebrant");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: keyword — "Convoke" is not in the usable KeywordAbility surface.
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
