//! Rat Colony — `{1}{B}` 2/1 Rat.
//! "This creature gets +1/+0 for each other Rat you control." (static
//!   dynamic P/T — no trigger/cost; GAP'd)
//! "A deck can have any number of cards named Rat Colony." (deckbuilding
//!   rule, not an ability — no implementation needed)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rat Colony");
    let rat = reg.interner_mut().intern("Rat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rat);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: static "+1/+0 for each other Rat you control" is a continuous
    // dynamic-P/T characteristic-defining boost with no trigger or cost.

    reg.register(CardDefinition::new(name, chars))
}
