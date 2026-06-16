//! Tromokratis — `{5}{U}{U}` 8/8 Legendary Kraken.
//! Tromokratis has hexproof unless it's attacking or blocking.
//! Tromokratis can't be blocked unless all creatures defending player controls
//! block it.
//!
//! Both lines are conditional static abilities (no trigger word, no cost) that
//! the demonstrated triggered/activated API cannot express:
//!   - conditional hexproof gated on combat status, and
//!   - the "can't be blocked unless ALL of the defending player's creatures
//!     block it" evasion clause.
//! Emitting them via CantBeBlocked/GrantKeyword would be unconditional and thus
//! materially wrong, so both are GAP'd. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tromokratis");
    let kraken = reg.interner_mut().intern("Kraken");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kraken);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        ..Default::default()
    };

    // GAP: "has hexproof unless attacking or blocking" — conditional static.
    // GAP: "can't be blocked unless all creatures defending player controls
    //      block it" — conditional evasion static.
    reg.register(CardDefinition::new(name, chars))
}
