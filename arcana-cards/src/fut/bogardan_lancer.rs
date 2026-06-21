//! Bogardan Lancer — `{1}{R}` 1/1 Human Knight with Flanking and Bloodthirst 1.
//!
//! Oracle:
//! * Bloodthirst 1 (If an opponent was dealt damage this turn, this creature
//!   enters with a +1/+1 counter on it.)
//! * Flanking.
//!
//! Both are base keyword characteristics handled by the engine's keyword
//! pipelines; nothing beyond listing them is required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bogardan Lancer");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Bloodthirst(1), KeywordAbility::Flanking],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
