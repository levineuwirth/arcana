//! Nightmare — `{5}{B}` */* Nightmare Horse with Flying.
//! "Nightmare's power and toughness are each equal to the number of
//! Swamps you control."
//!
//! Flying is an engine keyword. The characteristic-defining ability
//! (CR 604.3) that sets P/T to the number of Swamps you control is a
//! static and is not expressible with the demonstrated primitives —
//! recorded as a GAP, with P/T left as `*`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nightmare");
    let nightmare = reg.interner_mut().intern("Nightmare");
    let horse = reg.interner_mut().intern("Horse");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nightmare);
    subtypes.0.insert(horse);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: static CDA — power and toughness each equal to the number of
        // Swamps you control. Left as `*`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
