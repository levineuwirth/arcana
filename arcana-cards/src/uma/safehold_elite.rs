//! Safehold Elite — `{1}{G/W}` 2/2 Elf Scout with Persist.
//! When Safehold Elite dies, if it had no -1/-1 counters on it, return
//! it to the battlefield under its owner's control with a -1/-1 counter.
//! (CR 702.79 — Persist.)
//!
//! The mana cost uses a hybrid symbol `{G/W}`; the card's actual colors
//! are green and white, so `ColorSet::green() | ColorSet::white()` is
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
    let name = reg.interner_mut().intern("Safehold Elite");
    let elf = reg.interner_mut().intern("Elf");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Persist],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
