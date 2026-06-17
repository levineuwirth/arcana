//! Hoverguard Observer — `{2}{U}{U}` 3/3 blue Drone.
//!
//! Oracle:
//! * Flying
//! * "This creature can block only creatures with flying." — a static
//!   blocking RESTRICTION. The demonstrated API has no Effect / static
//!   primitive for "can block only [filter]" (it has ForbidBlocking and
//!   CantBeBlocked, but not a restriction on WHAT this creature may
//!   block), so that clause is GAP'd here. Flying is emitted faithfully.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hoverguard Observer");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: static "can block only creatures with flying" — no
        // block-restriction primitive in the demonstrated API.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
