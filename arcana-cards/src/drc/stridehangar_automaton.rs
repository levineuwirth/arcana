//! Stridehangar Automaton — `{3}` 1/4 Artifact Creature — Construct.
//! Thopters you control get +1/+1. (static — GAP)
//! If one or more artifact tokens would be created under your control,
//! those tokens plus an additional 1/1 colorless Thopter artifact creature
//! token with flying are created instead. (token-creation replacement — GAP)
//!
//! Both lines are pure static abilities (a continuous anthem and a
//! token-creation replacement effect) with no triggered/activated form
//! expressible in this card class — emitted as a vanilla artifact creature
//! with the statics GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stridehangar Automaton");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
