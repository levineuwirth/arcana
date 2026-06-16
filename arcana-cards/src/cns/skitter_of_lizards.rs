//! Skitter of Lizards — `{R}` 1/1 Lizard with Haste.
//! "Multikicker {1}{R}. This creature enters with a +1/+1 counter on it
//! for each time it was kicked."
//!
//! Haste is a base keyword. Multikicker is not an available
//! KeywordAbility variant, and the kicker count needed for the
//! enters-with-counters trigger is not modeled, so both are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skitter of Lizards");
    let lizard = reg.interner_mut().intern("Lizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: Multikicker {1}{R} — no Multikicker KeywordAbility variant and
    // no kicker-count accessor, so "enters with a +1/+1 counter for each
    // time it was kicked" cannot be expressed.
    reg.register(CardDefinition::new(name, chars))
}
