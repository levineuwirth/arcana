//! Leonardo, Big Brother — `{2}{W}` 1/3 Legendary Mutant Ninja Turtle.
//! "Sneak {W}" — GAP (an alternative-cast keyword not in the usable
//!   KeywordAbility surface; no alternative-cost cast primitive).
//! "Leonardo gets +1/+0 for each other creature you control." — GAP
//!   (static continuous self-anthem scaling with board count; not a
//!   triggered/activated ability).
//!
//! Both non-bones lines are unexpressible, so only the bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Leonardo, Big Brother");
    let mutant = reg.interner_mut().intern("Mutant");
    let ninja = reg.interner_mut().intern("Ninja");
    let turtle = reg.interner_mut().intern("Turtle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(ninja);
    subtypes.0.insert(turtle);

    // GAP: "Sneak {W}" — alternative-cast keyword, unmodeled.
    // GAP: "Leonardo gets +1/+0 for each other creature you control." —
    // dynamic static self-pump, not a triggered/activated ability.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
