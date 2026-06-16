//! Blast-Furnace Hellkite — `{7}{R}{R}` 5/5 red Dragon.
//!
//! Oracle:
//! * "Artifact offering (…)" — the Offering alternative-cast mechanic is
//!   not modeled by the engine. GAP (no usable `KeywordAbility` variant /
//!   cast hook).
//! * Flying, double strike. — both evergreen keywords, emitted in the
//!   keyword vec.
//! * "Creatures attacking your opponents have double strike." — a pure
//!   static continuous ability granting double strike to a board-wide
//!   filtered set of creatures; not expressible here. GAP.
//!
//! Only bones plus the two keywords are emitted; the two unsupported
//! clauses are noted as GAP comments.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blast-Furnace Hellkite");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    // GAP: "Artifact offering" — the Offering alternative-cast keyword is
    // not an expressible KeywordAbility variant and has no cast hook.

    // GAP: static "Creatures attacking your opponents have double strike."
    // — a board-wide filtered keyword grant; not expressible as a
    // triggered/activated ability.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::DoubleStrike],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
