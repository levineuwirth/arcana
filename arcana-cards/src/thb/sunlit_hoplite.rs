//! Sunlit Hoplite — `{1}{W}` 2/1 Human Soldier.
//!
//! "During your turn, this creature has first strike." and "This creature gets
//! +1/+0 as long as you control an Elspeth planeswalker." are both conditional
//! static continuous abilities with no trigger word or activation cost, so
//! neither is expressible here; only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sunlit Hoplite");
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
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "During your turn, this creature has first strike." — conditional static.
    // GAP: "This creature gets +1/+0 as long as you control an Elspeth planeswalker." — conditional static.
    reg.register(CardDefinition::new(name, chars))
}
