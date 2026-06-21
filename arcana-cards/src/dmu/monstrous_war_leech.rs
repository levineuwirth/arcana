//! Monstrous War-Leech — `{3}{B}` */* Leech Horror.
//!
//! Oracle:
//! * Kicker {U} (additional cost — GAP)
//! * As this creature enters, if it was kicked, mill four cards. (kicked
//!   condition — GAP)
//! * Monstrous War-Leech's power and toughness are each equal to the greatest
//!   mana value among cards in your graveyard. (CDA — GAP)
//!
//! None of this card's text is expressible with the available primitives:
//! Kicker has no cost field, the "if it was kicked" ETB condition has no
//! predicate, and the */* characteristic-defining ability has no
//! greatest-mana-value primitive and no star/dynamic PtValue variant. The
//! base P/T is recorded as Fixed(0) (the */* base) and every ability is GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Monstrous War-Leech");
    let leech = reg.interner_mut().intern("Leech");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leech);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: CDA — P/T equal to greatest mana value among graveyard cards;
        // no greatest-mana-value primitive and no star/dynamic PtValue.
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: keyword/cost — Kicker {U} has no additional-cost field.
    // GAP: ETB — "if it was kicked, mill four" has no kicked-condition predicate.
    reg.register(CardDefinition::new(name, chars))
}
