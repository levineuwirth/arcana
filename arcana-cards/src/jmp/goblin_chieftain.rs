//! Goblin Chieftain — `{1}{R}{R}` 2/2 Goblin with Haste.
//!
//! Oracle:
//! * Haste — keyword.
//! * Other Goblin creatures you control get +1/+1 and have haste. — a
//!   static continuous anthem. No `Effect` / ability shape models a
//!   passive board-wide buff on this card class, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Chieftain");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: static — "Other Goblin creatures you control get +1/+1 and have
    // haste." A passive continuous anthem; not expressible as a
    // triggered/activated ability on this card class.
    reg.register(CardDefinition::new(name, chars))
}
