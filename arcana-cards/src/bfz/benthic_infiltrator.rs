//! Benthic Infiltrator — `{2}{U}` 1/4 colorless (Devoid) Eldrazi Drone.
//! Ingest (combat damage to a player → that player exiles the top card of
//! their library) and "This creature can't be blocked."
//!
//! Devoid → the card is colorless. Ingest and the can't-be-blocked static are
//! GAP'd (see notes below).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Benthic Infiltrator");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        ..Default::default()
    };

    // GAP: Ingest ("deals combat damage to a player → that player exiles the
    // top card of their library") — there is no exile-top-of-library effect
    // targeting a player (Mill goes to graveyard, not exile).
    // GAP: "This creature can't be blocked" is a pure can't-be-blocked static
    // with no trigger/activation hook to attach CantBeBlocked to.
    reg.register(CardDefinition::new(name, chars))
}
