//! Souls of the Lost — `{1}{B}` */*+1 Spirit.
//! "As an additional cost to cast this spell, discard a card or sacrifice a
//!  permanent. Fathomless descent — Souls of the Lost's power is equal to the
//!  number of permanent cards in your graveyard and its toughness is equal to
//!  that number plus 1."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Souls of the Lost");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // `*/*+1` — power is a CDA (`*`), toughness is `*+1`.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::StarPlus(1)),
        ..Default::default()
    };
    // GAP: "As an additional cost to cast this spell, discard a card or sacrifice
    // a permanent." — additional cast cost, not a triggered/activated ability and
    // not expressible on this card class.
    // GAP: "Fathomless descent — power equals permanent cards in your graveyard,
    // toughness equals that number plus 1." — a characteristic-defining ability;
    // no static CDA primitive is available here, so the `*`/`*+1` printed values
    // remain unresolved.
    reg.register(CardDefinition::new(name, chars))
}
