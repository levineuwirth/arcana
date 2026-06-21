//! Vengeful Vampire — `{4}{B}{B}` 3/2 Creature — Vampire.
//!
//! Oracle:
//! * Flying.
//! * Undying (when it dies with no +1/+1 counters, return it with one).
//!
//! Both are keyword-line abilities; the engine implements the Undying
//! rules from the keyword marker, so no extra wiring is required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vengeful Vampire");
    let vampire = reg.interner_mut().intern("Vampire");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Undying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
