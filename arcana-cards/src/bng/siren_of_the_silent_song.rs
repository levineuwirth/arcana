//! Siren of the Silent Song — `{1}{U}{B}` 2/1 Zombie Siren.
//!
//! * Flying (keyword).
//! * Inspired — "Whenever this creature becomes untapped, each opponent
//!   discards a card, then each opponent mills a card." GAP: there is no
//!   "becomes untapped" trigger condition (only `SelfBecomesTapped`), so
//!   the entire Inspired ability is unexpressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Siren of the Silent Song");
    let zombie = reg.interner_mut().intern("Zombie");
    let siren = reg.interner_mut().intern("Siren");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(siren);

    // GAP: Inspired ("Whenever this creature becomes untapped, …") — no
    // becomes-untapped trigger condition exists in the engine.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
