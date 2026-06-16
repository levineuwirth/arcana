//! Kithkin Rabble — `{3}{W}` */* Kithkin with Vigilance.
//! "Kithkin Rabble's power and toughness are each equal to the number of white
//! permanents you control."
//!
//! Vigilance is a base keyword. The characteristic-defining */* (a dynamic
//! power/toughness set to a board count) has no `PtValue` variant in the
//! demonstrated API (only `PtValue::Fixed`), so the base P/T is set to 0/0 as a
//! placeholder and the CDA static is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kithkin Rabble");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP static CDA: P/T are each equal to the number of white permanents
        //      you control; no dynamic PtValue variant — placeholder 0/0.
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
