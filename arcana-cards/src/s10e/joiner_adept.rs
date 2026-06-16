//! Joiner Adept — `{1}{G}` 2/1 green Elf Druid.
//! "Lands you control have '{T}: Add one mana of any color.'"
//! GAP: Granting an activated ability to all permanents of a type (continuous
//! effect, layer 6) is not expressible as a triggered or activated ability.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Joiner Adept");
    let elf = reg.interner_mut().intern("Elf");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(druid);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    // GAP: "Lands you control have '{T}: Add one mana of any color'" is a continuous
    // effect that grants an activated ability to a class of permanents. This requires
    // a WhileSourceOnBattlefield layer effect not modeled in the engine's card shape.
    reg.register(CardDefinition::new(name, chars))
}
