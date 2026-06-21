//! Nef-Crop Entangler — `{1}{R}` 2/1 Creature — Human Warrior.
//!
//! Oracle:
//! * Trample
//! * You may exert this creature as it attacks. When you do, it gets +1/+2
//!   until end of turn.
//!
//! Exert is not in the demonstrated KeywordAbility surface, and the "exert as it
//! attacks → +1/+2" coupling has no demonstrated trigger condition or cost
//! field (the exert choice on attack is engine debt). The whole exert ability is
//! GAP'd; only Trample is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nef-Crop Entangler");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: Exert + "as it attacks, it gets +1/+2" — exert is not in the keyword
    // surface and has no demonstrated trigger/cost hook.
    reg.register(CardDefinition::new(name, chars))
}
