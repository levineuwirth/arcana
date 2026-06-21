//! Haktos the Unscarred — `{R}{R}{W}{W}` Legendary 6/1 Human Warrior.
//!
//! * Haktos attacks each combat if able. GAP: there is no "must attack" static
//!   restriction primitive.
//! * As Haktos enters, choose 2, 3, or 4 at random. GAP: no "choose a number at
//!   random" mechanism, and the chosen number can't be stored.
//! * Haktos has protection from each mana value other than the chosen number.
//!   GAP: Protection (and protection keyed to mana value) is not expressible.
//!
//! All three clauses are gaps; only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Haktos the Unscarred");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{R}{W}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
