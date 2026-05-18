//! Gravelgill Axeshark — `{4}{U/B}` 3/3 Merfolk Soldier with Persist.
//! When Gravelgill Axeshark dies, if it had no -1/-1 counters on it,
//! return it to the battlefield under its owner's control with a -1/-1
//! counter. (CR 702.79 — Persist.)
//!
//! The mana cost uses a hybrid symbol `{U/B}`; the card's actual colors
//! are blue and black, so `ColorSet::blue() | ColorSet::black()` is
//! used per the engine conventions.
//!
//! # Rules references
//!
//! * CR 702.79 — Persist. When this permanent dies, if it had no -1/-1
//!   counters on it, return it to the battlefield under its owner's
//!   control with a -1/-1 counter on it.
//!
//! Persist is a fully implemented unit-variant keyword; listing
//! `KeywordAbility::Persist` in `keywords` is sufficient — the runtime
//! pipeline handles the return-from-graveyard trigger.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gravelgill Axeshark");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U/B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Persist],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
