//! Ixidor, Reality Sculptor — `{3}{U}{U}` 3/4 Legendary Human Wizard.
//!
//! * Face-down creatures get +1/+1. (Static anthem scoped to face-down
//!   creatures; no expressible face-down filter / conditional anthem.
//!   GAP'd.)
//! * {2}{U}: Turn target face-down creature face up. (No `TurnFaceUp`
//!   effect primitive and no face-down target filter. GAP'd.)
//!
//! Both abilities require unavailable primitives; the card carries only
//! its bones.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ixidor, Reality Sculptor");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
