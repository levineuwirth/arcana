//! Star Whale — `{6}{U}{U}` 8/8 Creature — Alien Whale.
//!
//! Oracle:
//! * "Flying, vigilance" — keywords.
//! * "Other creatures you control have ward {2}." — a static ability
//!   granting Ward to OTHER creatures you control. GAP: there is no
//!   demonstrated primitive for a continuous keyword-granting anthem
//!   (not a triggered/activated ability).
//! * "Suspend 6—{1}{U}" — Suspend is NOT in the usable KeywordAbility
//!   surface (cast-time exile-with-counters mechanic). GAP: keyword
//!   omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Star Whale");
    let alien = reg.interner_mut().intern("Alien");
    let whale = reg.interner_mut().intern("Whale");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(whale);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "Other creatures you control have ward {2}" (keyword-
    // granting anthem) is not expressible.
    // GAP: Suspend 6—{1}{U} not expressible (not in usable keyword surface).
    reg.register(CardDefinition::new(name, chars))
}
