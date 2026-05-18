//! Suntouched Myr — `{3}` 0/0 Artifact Creature — Myr with Sunburst.
//!
//! Fifth Dawn (2004). Sunburst causes it to enter with a +1/+1 counter for
//! each color of mana spent to cast it; printed P/T is 0/0.
//!
//! # Rules references
//!
//! * CR 702.44 — Sunburst. Counts distinct colors among mana spent to cast
//!   the spell; each color adds one +1/+1 counter (on creatures) or charge
//!   counter (on non-creatures). Engine wiring handles the ETB counter
//!   placement.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Suntouched Myr");
    let myr = reg.interner_mut().intern("Myr");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(myr);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: (TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Sunburst],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
