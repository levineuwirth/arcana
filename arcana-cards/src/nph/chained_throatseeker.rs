//! Chained Throatseeker — `{5}{U}` 5/5 Creature — Phyrexian Horror with Infect.
//!
//! Infect.
//! This creature can't attack unless defending player is poisoned.
//!
//! Infect is a base keyword. The "can't attack unless defending player is
//! poisoned" line is a STATIC attack restriction (no trigger word, no cost) —
//! not expressible as a triggered/activated ability, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chained Throatseeker");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(horror);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Infect],
        // GAP: static "can't attack unless defending player is poisoned" — a
        // continuous attack restriction; not expressible here.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
