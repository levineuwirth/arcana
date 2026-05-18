//! Lost Leonin — `{1}{W}` 2/1 Phyrexian Cat Soldier with Infect.
//! New Phyrexia common; a cheap white infect creature used in
//! aggressive poison-counter strategies.
//!
//! # Rules references
//!
//! * CR 702.90 — Infect. This creature deals damage to creatures in
//!   the form of -1/-1 counters and to players in the form of poison
//!   counters. Engine wiring handles both substitution effects.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lost Leonin");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let cat = reg.interner_mut().intern("Cat");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(cat);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Infect],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
