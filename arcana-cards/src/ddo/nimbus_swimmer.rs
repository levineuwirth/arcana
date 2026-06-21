//! Nimbus Swimmer — `{X}{G}{U}` 0/0 Leviathan.
//! Flying.
//! This creature enters with X +1/+1 counters on it.
//!
//! Flying is a base keyword. "Enters with X +1/+1 counters" has no
//! documented enters-with-X primitive in the available effect surface
//! (the cast-time X is not exposed to a SelfEntersBattlefield resolver),
//! so it is GAP'd — matching the established catalog precedent
//! (Wren's Run Hydra).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nimbus Swimmer");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leviathan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "This creature enters with X +1/+1 counters on it." — no
    //      enters-with-X primitive exposes the cast X to the ETB.
    reg.register(CardDefinition::new(name, chars))
}
