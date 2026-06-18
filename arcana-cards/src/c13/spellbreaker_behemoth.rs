//! Spellbreaker Behemoth — `{1}{R}{G}{G}` 5/5 Beast (G/R).
//!
//! "This spell can't be countered.
//!  Creature spells you control with power 5 or greater can't be countered."
//!
//! Both clauses are can't-be-countered statics (one self-applying to this
//! spell, one board-wide on your big creature spells). Neither is a
//! triggered or activated ability and there is no expressible can't-be-
//! countered primitive — both are GAP'd. Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Spellbreaker Behemoth");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}{G}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP (static): "This spell can't be countered."
    // GAP (static): "Creature spells you control with power 5 or greater
    //      can't be countered."
    reg.register(CardDefinition::new(name, chars))
}
