//! Knight of the Pilgrim's Road — `{2}{W}` 3/2 Human Knight with Renown 1.
//! Magic Origins common; a three-mana white creature that grows
//! permanently after dealing combat damage to a player via Renown.
//!
//! # Rules references
//!
//! * CR 702.112 — Renown. When this creature deals combat damage to a
//!   player, if it isn't renowned, put a +1/+1 counter on it and it
//!   becomes renowned. The N parameter (here 1) is the number of
//!   +1/+1 counters placed.
//!
//! Renown is a fully-implemented parametrized keyword; listing
//! `KeywordAbility::Renown(1)` in `keywords` is sufficient — the
//! runtime pipeline handles the trigger.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Knight of the Pilgrim's Road");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Renown(1)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
