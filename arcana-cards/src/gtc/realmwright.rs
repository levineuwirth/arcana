//! Realmwright — `{U}` 1/1 Vedalken Wizard.
//! "As this creature enters, choose a basic land type. Lands you control are
//! the chosen type in addition to their other types." Both are an as-enters
//! choice + continuous static type grant — not expressible (GAP).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Realmwright");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "As this creature enters, choose a basic land type" (replacement
    // choice) and the continuous "Lands you control are the chosen type" static
    // type-grant are not expressible with the available primitives.
    reg.register(CardDefinition::new(name, chars))
}
