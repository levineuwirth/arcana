//! Wilt-Leaf Liege — `{1}{G/W}{G/W}{G/W}` 4/4 Elf Knight (G/W).
//! All three lines are statics with no triggered/activated form: two anthem
//! continuous effects (other green / other white creatures you control get
//! +1/+1) and a discard-replacement effect (if an opponent's spell/ability makes
//! you discard this, put it onto the battlefield instead). Bones-only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wilt-Leaf Liege");
    let elf = reg.interner_mut().intern("Elf");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G/W}{G/W}{G/W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static anthem "Other green creatures you control get +1/+1."
    // GAP: static anthem "Other white creatures you control get +1/+1."
    // GAP: replacement "if an opponent's spell/ability makes you discard this,
    //      put it onto the battlefield instead."
    reg.register(CardDefinition::new(name, chars))
}
