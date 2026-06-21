//! Corpulent Corpse — `{5}{B}` 3/3 Zombie with Fear.
//!
//! Oracle:
//! * Fear
//! * Suspend 5—{B} (GAP — Suspend is a cast-alternative mechanic with
//!   no expressible KeywordAbility variant or hook.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Corpulent Corpse");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    // GAP: "Suspend 5—{B}" — Suspend is a cast-alternative mechanic with
    // no expressible KeywordAbility variant.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Fear],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
