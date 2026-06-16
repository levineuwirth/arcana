//! Thalia, Guardian of Thraben — `{1}{W}` 2/1 Legendary Human Soldier.
//!
//! * First strike — base keyword line (`KeywordAbility::FirstStrike`).
//! * "Noncreature spells cost {1} more to cast." is a pure static
//!   cost-increasing ability — no trigger word, no activation cost, and no
//!   demonstrated primitive expresses a cost-modification static. GAP'd.
//!
//! With only a static beyond the keyword, this card carries no triggered or
//! activated abilities; the bones + First strike are emitted faithfully.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thalia, Guardian of Thraben");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    // GAP: static — "Noncreature spells cost {1} more to cast." No demonstrated
    // cost-modification primitive.

    reg.register(CardDefinition::new(name, chars))
}
