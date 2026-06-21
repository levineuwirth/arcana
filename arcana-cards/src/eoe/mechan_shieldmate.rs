//! Mechan Shieldmate — `{1}{U}` 3/2 blue Artifact Creature — Robot Soldier.
//! Defender.
//! As long as an artifact entered the battlefield under your control this
//! turn, this creature can attack as though it didn't have defender.
//!
//! The conditional defender-bypass static has no expressible form (no
//! "can attack as though it lacked defender" effect / static). GAP that
//! line; Defender keyword is recorded.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mechan Shieldmate");
    let robot = reg.interner_mut().intern("Robot");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot);
    subtypes.0.insert(soldier);

    // GAP: static "can attack as though it didn't have defender while an
    // artifact entered under your control this turn" — not expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
