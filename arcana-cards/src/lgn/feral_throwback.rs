//! Feral Throwback — `{4}{G}{G}` 3/3 Beast with Amplify 2 and Provoke.
//! * Amplify 2 — as it enters, put two +1/+1 counters on it per Beast
//!   card revealed from hand.
//! * Provoke — whenever it attacks, may force a creature to block.
//! Both are fully-modeled keyword abilities.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feral Throwback");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Amplify(2), KeywordAbility::Provoke],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
