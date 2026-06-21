//! Glittering Lynx — `{W}` 1/1 Cat.
//!
//! Oracle:
//! * Prevent all damage that would be dealt to this creature. (static
//!   self-prevention — GAP: not a triggered/activated ability)
//! * `{2}`: Until end of turn, this creature loses "Prevent all damage
//!   that would be dealt to this creature." Any player may activate this
//!   ability. (GAP — strips a specific named ability + any-player
//!   activation, neither expressible)
//!
//! Both abilities are GAP'd; a faithful 1/1 Cat body is registered.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glittering Lynx");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "Prevent all damage that would be dealt to this creature." (static)
    // GAP: "{2}: ... this creature loses [the prevention ability]. Any player
    //       may activate this ability." — named-ability removal + any-player
    //       activation are not expressible.
    reg.register(CardDefinition::new(name, chars))
}
