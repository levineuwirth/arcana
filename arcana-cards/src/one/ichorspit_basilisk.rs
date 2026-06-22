//! Ichorspit Basilisk — `{2}{G}` 1/3 Phyrexian Basilisk with Deathtouch
//! and Toxic 1.
//!
//! Oracle:
//! * Deathtouch.
//! * Toxic 1 (players dealt combat damage by this creature also get a
//!   poison counter).
//!
//! Both are base-characteristic keywords.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ichorspit Basilisk");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let basilisk = reg.interner_mut().intern("Basilisk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(basilisk);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch, KeywordAbility::Toxic(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
