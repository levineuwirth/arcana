//! Mister Cheddar, Cheese Sliver — `{4}{B}` 5/5 Legendary Rat Sliver.
//! Rats and Slivers you control are Rat Slivers in addition to their other
//! types and have ratmanship.
//! As long as Mister Cheddar is your commander, Sliver cards in your deck have
//! "Your deck may contain any number of this card."

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mister Cheddar, Cheese Sliver");
    let rat = reg.interner_mut().intern("Rat");
    let sliver = reg.interner_mut().intern("Sliver");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);
    subtypes.0.insert(sliver);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "Rats and Slivers you control are Rat Slivers in addition
    // to their other types and have ratmanship" is a continuous type/keyword
    // grant with a custom keyword (ratmanship) not in the usable surface.
    // GAP: static — "as long as Mister Cheddar is your commander, Sliver cards
    // in your deck have ..." is a deck-construction static, not expressible.
    reg.register(CardDefinition::new(name, chars))
}
