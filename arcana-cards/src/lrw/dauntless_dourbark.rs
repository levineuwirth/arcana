//! Dauntless Dourbark — `{3}{G}` */* green Treefolk Warrior.
//! Its power and toughness are each equal to the number of Forests you
//! control plus the number of Treefolk you control.
//! This creature has trample as long as you control another Treefolk.
//!
//! Both lines are pure continuous statics (no trigger word, no cost): a
//! characteristic-defining ability for the */* P/T and a conditional
//! trample grant. Neither is expressible via the MultiAbilityCreature
//! triggered/activated surface. Base P/T recorded as 0/0 (the printed */*).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dauntless Dourbark");
    let treefolk = reg.interner_mut().intern("Treefolk");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(treefolk);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        // GAP: P/T is a characteristic-defining static (Forests + Treefolk
        // you control); printed */* recorded as base 0/0.
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    // GAP: static "has trample as long as you control another Treefolk" is a
    // conditional continuous keyword grant, not a triggered/activated ability.
    reg.register(CardDefinition::new(name, chars))
}
