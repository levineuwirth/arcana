//! Charging War Boar — `{1}{R}{G}` 3/1 Boar (R/G) with Haste.
//! Static: while you control a Domri planeswalker, gets +1/+1 and has
//! trample. (GAP — conditional static pump + keyword grant.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charging War Boar");
    let boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        // GAP: conditional static "+1/+1 and trample while you control a
        // Domri planeswalker" not expressible here.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
