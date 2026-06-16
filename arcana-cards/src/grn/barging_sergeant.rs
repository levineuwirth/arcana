//! Barging Sergeant — `{4}{R}` 4/2 Minotaur Soldier with Haste and
//! Mentor. Pure keyword line; no triggered/activated abilities beyond
//! the keyword-synthesized Mentor trigger.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barging Sergeant");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(minotaur);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste, KeywordAbility::Mentor],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
