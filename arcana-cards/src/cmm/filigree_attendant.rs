//! Filigree Attendant — `{2}{U}{U}` */3 Artifact Creature — Homunculus.
//!
//! Oracle:
//! * Flying.
//! * "Filigree Attendant's power is equal to the number of artifacts you
//!   control." — a characteristic-defining ability; power is marked `*`
//!   (PtValue::Star). The CDA that computes the value is a continuous
//!   static, not a triggered/activated ability, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Filigree Attendant");
    let homunculus = reg.interner_mut().intern("Homunculus");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(homunculus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: CDA "power is equal to the number of artifacts you control" — a
    // continuous characteristic-defining static (power marked `*`), not a
    // triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
