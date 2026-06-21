//! Earthen Goo — `{2}{R}` 2/2 Ooze with Trample.
//!
//! Oracle:
//! * Trample.
//! * Cumulative upkeep {R} or {G}.
//! * This creature gets +1/+1 for each age counter on it.
//!
//! Trample is a base characteristic. Cumulative upkeep is not a usable keyword
//! and its "put an age counter, then sacrifice unless you pay per counter"
//! mechanic has no expressible upkeep trigger / sacrifice-unless-pay shape with
//! a per-counter cost. The dynamic self-pump keyed off the age counters is a
//! static continuous effect with no primitive. Both are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Earthen Goo");
    let ooze = reg.interner_mut().intern("Ooze");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ooze);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: Cumulative upkeep {R} or {G} — not a usable keyword; the
    // age-counter-then-sacrifice-unless-pay upkeep mechanic with a per-counter
    // mana cost is not expressible.
    // GAP: "gets +1/+1 for each age counter on it" — self-referential static
    // continuous pump; no primitive.
    reg.register(CardDefinition::new(name, chars))
}
