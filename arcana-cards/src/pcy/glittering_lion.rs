//! Glittering Lion — `{2}{W}` 2/2 Cat.
//!
//! Prevent all damage that would be dealt to this creature.
//! {3}: Until end of turn, this creature loses "Prevent all damage that would
//! be dealt to this creature." Any player may activate this ability.
//!
//! The static damage-prevention shield and the activation that suspends it are
//! intertwined self-replacement mechanics not expressible with the demonstrated
//! API; both are GAP'd, leaving bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glittering Lion");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static "prevent all damage that would be dealt to this creature" and
    // the {3} ability that suspends it ("Any player may activate") — the
    // self-referential prevention shield + shared activation aren't expressible.
    reg.register(CardDefinition::new(name, chars))
}
