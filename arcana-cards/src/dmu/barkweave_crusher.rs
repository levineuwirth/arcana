//! Barkweave Crusher — `{3}{G}` 2/5 Creature — Elemental Warrior with Enlist.
//! A green enlist creature that can tap a non-attacking ally to temporarily
//! add its power to the Crusher's for the turn.
//!
//! # Rules references
//!
//! * CR 702.154 — Enlist. As this creature attacks, you may tap a nonattacking
//!   creature you control without summoning sickness. When you do, add its
//!   power to this creature's until end of turn. The runtime enlist pipeline
//!   handles the tap and the power boost; listing the keyword in `keywords`
//!   is all that is required here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barkweave Crusher");
    let elemental = reg.interner_mut().intern("Elemental");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Enlist],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
