//! Bloodsworn Steward — `{2}{R}{R}` 4/4 Vampire Knight.
//!
//! Oracle:
//! * Flying
//! * Commander creatures you control get +2/+2 and have haste. (static
//!   commander-restricted anthem — GAP)
//!
//! Flying is wired; the commander anthem is GAP'd (no commander-filtered
//! static buff via the demonstrated API).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodsworn Steward");
    let vampire = reg.interner_mut().intern("Vampire");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Commander creatures you control get +2/+2 and have haste."
    //      (static commander-restricted anthem)
    reg.register(CardDefinition::new(name, chars))
}
