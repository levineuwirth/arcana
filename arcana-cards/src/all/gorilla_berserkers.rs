//! Gorilla Berserkers — `{3}{G}{G}` 2/3 Ape Berserker with Trample and Rampage 2.
//! Can't be blocked except by three or more creatures.
//!
//! Trample and Rampage 2 are wired. The "can't be blocked except by three or
//! more creatures" static (a Menace-3 variant) has no expressible primitive.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gorilla Berserkers");
    let ape = reg.interner_mut().intern("Ape");
    let berserker = reg.interner_mut().intern("Berserker");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ape);
    subtypes.0.insert(berserker);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Rampage(2)],
        ..Default::default()
    };

    // GAP: static — "can't be blocked except by three or more creatures"
    // (Menace requires only two); no parametrized block-count primitive.

    reg.register(CardDefinition::new(name, chars))
}
