//! Untamed Kavu — `{1}{G}` 2/2 Kavu with Vigilance and Trample.
//!
//! Oracle text:
//!  * Kicker {3} — "You may pay an additional {3} as you cast this spell."
//!  * "Vigilance, trample"
//!  * "If this creature was kicked, it enters with three +1/+1 counters
//!    on it."
//!
//! Vigilance and Trample are base keywords. Kicker is not a supported
//! `KeywordAbility` variant and there is no cast-time additional-cost
//! mechanism, so the kicker and its "if kicked, enters with three +1/+1
//! counters" rider are GAP'd (the ETB conditional cannot be expressed
//! without a "was kicked" predicate).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Untamed Kavu");
    let kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);

    // GAP: keyword — Kicker {3} is not a supported KeywordAbility variant,
    // and the "if this creature was kicked, it enters with three +1/+1
    // counters on it" ETB rider has no "was kicked" predicate to gate on.
    // Both the kicker cost and its conditional ETB are omitted.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
