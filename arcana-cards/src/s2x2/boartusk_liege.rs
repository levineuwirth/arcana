//! Boartusk Liege — `{1}{R/G}{R/G}{R/G}` 3/4 Goblin Knight with Trample.
//! "Other red creatures you control get +1/+1."
//! "Other green creatures you control get +1/+1."
//!
//! Trample is a base keyword. Both lord-style continuous anthems are pure
//! statics with no triggered/activated form expressible in this class — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boartusk Liege");
    let goblin = reg.interner_mut().intern("Goblin");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R/G}{R/G}{R/G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static anthem "Other red creatures you control get +1/+1".
    // GAP: static anthem "Other green creatures you control get +1/+1".
    reg.register(CardDefinition::new(name, chars))
}
