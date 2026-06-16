//! Relentless Raptor — `{R}{W}` 3/3 Creature — Dinosaur. Vigilance.
//!
//! Oracle: "Vigilance. This creature attacks or blocks each combat if able."
//!
//! Only the Vigilance keyword is expressible. The "attacks or blocks each
//! combat if able" clause is a pure static combat-requirement (CR 508.1c /
//! 509.1c style) with no trigger word and no activation cost — it is not a
//! triggered/activated ability and has no Effect-catalog representation.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Relentless Raptor");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    // GAP: static "This creature attacks or blocks each combat if able" — a
    // pure static combat-requirement, not a triggered/activated ability and
    // not expressible with the documented Effect catalog.
    reg.register(CardDefinition::new(name, chars))
}
