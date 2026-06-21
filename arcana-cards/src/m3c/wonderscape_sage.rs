//! Wonderscape Sage — `{1}{U}` 1/3 Creature — Moonfolk Wizard.
//!
//! * Flying (keyword).
//! * "{T}, Return a land you control to its owner's hand: Draw a card. Then
//!   discard a card unless that land had a nonbasic land type." — the
//!   additional cost "Return a land you control to its owner's hand" is not an
//!   `ActivationCost` field (there is no return-a-permanent cost), and the
//!   conditional discard depends on the returned land's subtype. The activated
//!   ability is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wonderscape Sage");
    let moonfolk = reg.interner_mut().intern("Moonfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moonfolk);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    // GAP: "{T}, Return a land you control to its owner's hand: Draw a card.
    // Then discard a card unless that land had a nonbasic land type." — the
    // return-a-land cost is not an ActivationCost field.
    reg.register(CardDefinition::new(name, chars))
}
