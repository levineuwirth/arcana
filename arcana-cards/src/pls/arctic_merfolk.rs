//! Arctic Merfolk — `{1}{U}` 1/1 Merfolk.
//!
//! Kicker—Return a creature you control to its owner's hand.
//! If this creature was kicked, it enters with a +1/+1 counter on it.
//!
//! Kicker is not an available `KeywordAbility` variant and the non-mana
//! "return a creature" kicker cost / "if it was kicked" enters-with rider
//! has no expressible primitive. Both clauses are GAP'd; the vanilla bones
//! are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arctic Merfolk");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);

    // GAP: keyword — Kicker (non-mana cost) is not an available KeywordAbility.
    // GAP: "If this creature was kicked, it enters with a +1/+1 counter" —
    //      no kicked-conditional enters-with primitive.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
