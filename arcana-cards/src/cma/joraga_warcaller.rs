//! Joraga Warcaller — `{G}` 1/1 Elf Warrior.
//! Multikicker {1}{G}; enters with a +1/+1 counter per kick; Other Elf
//! creatures you control get +1/+1 for each +1/+1 counter on this creature.
//!
//! GAP: Multikicker is not a usable keyword, so the kick-driven enters-with-N-
//! counters clause is unexpressible, and the static Elf anthem ("get +1/+1 for
//! each +1/+1 counter on this creature") is a continuous effect, not a
//! triggered/activated ability. Only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Joraga Warcaller");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // Multikicker is not a usable keyword.
        keywords: vec![],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
