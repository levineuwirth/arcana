//! Bronze Guardian — `{4}{W}` */5 Artifact Creature — Golem.
//! "Double strike
//!  Ward {2}
//!  Other artifacts you control have ward {2}.
//!  Bronze Guardian's power is equal to the number of artifacts you
//!  control."
//!
//! Double strike and Ward {2} are keywords. "Other artifacts you control
//! have ward {2}" is a static keyword-granting buff with no demonstrated
//! hook on this shape — GAP. The characteristic-defining "power equal to
//! the number of artifacts you control" has no CDA registration hook, so
//! power is left as `*` (PtValue::Star) and GAP'd.

// GAP (CDA): "power is equal to the number of artifacts you control" — no
// registration hook for a characteristic-defining power static.
// GAP (static): "Other artifacts you control have ward {2}" — no
// keyword-granting anthem hook on this shape.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bronze Guardian");
    let golem = reg.interner_mut().intern("Golem");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(golem);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::DoubleStrike,
            KeywordAbility::Ward(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
