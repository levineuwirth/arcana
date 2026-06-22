//! Sumala Rumblers — `{2}{G/W}{G/W}` */4 Wurm.
//! "Sumala Rumblers's power is equal to the number of creatures you
//!   control." — GAP (characteristic-defining static; power modeled as
//!   `*`).
//! Myriad — GAP (not a modeled KeywordAbility; the attack-time token-copy
//!   mechanic is unexpressible).
//!
//! This card's only non-bones text is a CDA static (power = creatures you
//! control) plus the Myriad keyword, neither of which is a triggered or
//! activated ability. Power is set to `*` (PtValue::Star), the established
//! idiom for a CDA; toughness is the printed 4.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sumala Rumblers");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    // Colors: G, W (the {G/W} hybrid pips make it green-white).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G/W}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: "power is equal to the number of creatures you control" —
        // a CDA; modeled as `*`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: Myriad — not a modeled KeywordAbility.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
