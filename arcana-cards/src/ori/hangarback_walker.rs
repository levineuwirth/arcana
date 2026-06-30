//! Hangarback Walker — `{X}{X}` Artifact Creature — Construct, 0/0 that enters
//! with X +1/+1 counters (EntersWithSpec::CountersFromX). (GAP: "when this
//! dies, make a 1/1 flying Thopter per +1/+1 counter" + "{1},{T}: put a +1/+1
//! counter" — modeled as an X/X body only.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::types::{CardId, CounterKind, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hangarback Walker");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{X}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_enters_with(EntersWithSpec::CountersFromX {
            kind: CounterKind::PlusOnePlusOne,
        }),
    )
}
