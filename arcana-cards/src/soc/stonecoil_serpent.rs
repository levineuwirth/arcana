//! Stonecoil Serpent — `{X}` 0/0 Artifact Creature — Snake.
//! "Reach, trample, protection from multicolored. This creature enters
//! with X +1/+1 counters on it."
//!
//! Reach and Trample are evergreen keywords; protection from multicolored
//! is NOT an expressible KeywordAbility variant (GAP'd). The "enters with X
//! +1/+1 counters" rider is `EntersWithSpec::CountersFromX`, which reads the
//! cast's X as Hangarback Walker's does (A1.5ar: an empty enters trigger
//! stood in for it, so the 0/0 died on entering).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stonecoil Serpent");
    let snake = reg.interner_mut().intern("Snake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(snake);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        // GAP: "protection from multicolored" — Protection is not an
        // expressible KeywordAbility variant.
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_enters_with(EntersWithSpec::CountersFromX {
            kind: CounterKind::PlusOnePlusOne,
        }),
    )
}
