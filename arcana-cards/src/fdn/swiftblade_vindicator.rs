//! Swiftblade Vindicator — `{R}{W}` 1/1 Human Soldier with Double
//! strike, Vigilance, and Trample.
//!
//! All three are evergreen unit keywords carried as base
//! characteristics; the runtime combat pipelines do the rest. There
//! are no triggered or activated abilities to decompose.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Swiftblade Vindicator");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::DoubleStrike,
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
