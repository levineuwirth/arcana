//! Battle Squadron — `{3}{R}{R}` */* red Goblin with Flying.
//! "Battle Squadron's power and toughness are each equal to the number of
//! creatures you control." (a characteristic-defining ability — left as a GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Battle Squadron");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);

    // GAP: "power and toughness are each equal to the number of creatures you
    // control" is a characteristic-defining ability (CDA). The MultiAbilityCreature
    // surface has no blessed way to wire a dynamic CDA P/T, so the printed */*
    // is recorded via PtValue::Star and the defining static is omitted.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
