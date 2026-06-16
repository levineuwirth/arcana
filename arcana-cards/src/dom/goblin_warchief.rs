//! Goblin Warchief — `{1}{R}{R}` 2/2 Goblin Warrior.
//! "Goblin spells you cast cost {1} less to cast. Goblins you control
//! have haste." Both lines are static continuous abilities (a
//! cost-reduction effect and a keyword-granting anthem) — neither is a
//! triggered or activated ability, so only the bones are emitted here.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Goblin Warchief");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static "Goblin spells you cast cost {1} less" — cost-reduction static, not a triggered/activated ability.
    // GAP: static "Goblins you control have haste" — keyword-granting anthem static, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
