//! Liu Bei, Lord of Shu — `{3}{W}{W}` 2/4 Legendary Human Soldier with
//! Horsemanship. "Liu Bei gets +2/+2 as long as you control a permanent
//! named Guan Yu, Sainted Warrior or a permanent named Zhang Fei, Fierce
//! Warrior." (static — GAP'd below.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

// GAP static: "Liu Bei gets +2/+2 as long as you control a permanent named
// Guan Yu / Zhang Fei" — a conditional continuous self-buff, not expressible
// as a triggered/activated ability in this card class.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liu Bei, Lord of Shu");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Horsemanship],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
