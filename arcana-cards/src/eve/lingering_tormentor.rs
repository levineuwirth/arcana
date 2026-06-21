//! Lingering Tormentor — `{3}{B}` 2/2 Spirit with Fear and Persist.
//!
//! Oracle:
//! * Fear (can't be blocked except by artifact and/or black creatures).
//! * Persist (when it dies, if it had no -1/-1 counters, return it with a
//!   -1/-1 counter).
//!
//! Both lines are keyword abilities with reminder text only; both are usable
//! KeywordAbility variants, so nothing beyond listing them is required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lingering Tormentor");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Fear, KeywordAbility::Persist],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
