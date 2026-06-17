//! Scheming Fence — `{W}{U}` 2/3 Human Citizen.
//!
//! Its entire rules text — choosing a nonland permanent as it enters, locking
//! out that permanent's activated abilities, and copying those abilities onto
//! itself with any-color mana — is a set of statics/replacements tied to a
//! chosen-permanent reference, none of which is expressible. Only bones emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Scheming Fence");
    let human = reg.interner_mut().intern("Human");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(citizen);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "As this creature enters, you may choose a nonland permanent." — chosen-permanent reference.
    // GAP: "Activated abilities of the chosen permanent can't be activated." — static lockout.
    // GAP: "This creature has all activated abilities of the chosen permanent ..." — ability copy + any-color mana.
    reg.register(CardDefinition::new(name, chars))
}
