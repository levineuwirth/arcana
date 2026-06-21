//! Camel — `{W}` 0/1 Creature — Camel with Banding.
//! "As long as this creature is attacking, prevent all damage Deserts would
//!  deal to this creature and to creatures banded with this creature."
//!
//! Banding is recorded as a keyword. The conditional, source-filtered
//! (Deserts) prevention static — gated on "while attacking" and extending to
//! banded creatures — is a static continuous ability not expressible with the
//! demonstrated API, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Camel");
    let camel = reg.interner_mut().intern("Camel");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(camel);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Banding],
        ..Default::default()
    };

    // GAP: "As long as this creature is attacking, prevent all damage Deserts
    // would deal to this creature and to creatures banded with this creature"
    // — a conditional, source-filtered prevention static is not expressible.

    reg.register(CardDefinition::new(name, chars))
}
