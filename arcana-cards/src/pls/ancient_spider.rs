//! Ancient Spider — `{2}{G}{W}` 2/5 green-white Spider with First Strike and
//! Reach.
//!
//! # Rules references
//!
//! * CR 702.7 — First strike. This creature deals combat damage before
//!   creatures without first strike.
//! * CR 702.17 — Reach. This creature can block creatures with flying.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ancient Spider");
    let spider = reg.interner_mut().intern("Spider");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spider);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::FirstStrike, KeywordAbility::Reach],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
