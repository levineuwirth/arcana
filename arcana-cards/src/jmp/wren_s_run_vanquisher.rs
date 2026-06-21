//! Wren's Run Vanquisher — `{1}{G}` 3/3 Elf Warrior with Deathtouch.
//! As an additional cost to cast this spell, reveal an Elf card from your hand
//! or pay {3}. (GAP: additional casting costs are not expressible.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wren's Run Vanquisher");
    let elf = reg.interner_mut().intern("Elf");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elf);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Deathtouch],
        // GAP: static — "As an additional cost to cast this spell, reveal an Elf
        // card from your hand or pay {3}" has no additional-cost mechanism.
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
