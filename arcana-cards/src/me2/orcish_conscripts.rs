//! Orcish Conscripts — `{R}` 2/2 Orc.
//! "This creature can't attack unless at least two other creatures attack.
//!  This creature can't block unless at least two other creatures block."
//!
//! Both lines are pure static combat-restrictions with a numeric
//! "unless at least two others" gate that the MultiAbilityCreature
//! surface (keywords / triggered / activated) cannot express. The card
//! reduces to its bones.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Orcish Conscripts");
    let orc = reg.interner_mut().intern("Orc");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(orc);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: static "can't attack unless at least two other creatures attack" — no expressible primitive.
    // GAP: static "can't block unless at least two other creatures block" — no expressible primitive.
    reg.register(CardDefinition::new(name, chars))
}
