//! Sanctum Prelate — `{1}{W}{W}` 2/2 Human Cleric.
//!
//! Oracle:
//! * As this creature enters, choose a number. (GAP — an "as enters"
//!   number-choice replacement with no expressible hook.)
//! * Noncreature spells with mana value equal to the chosen number
//!   can't be cast. (GAP — a static cast-restriction; not a
//!   triggered/activated ability.)
//!
//! Neither clause is a triggered or activated ability, so only the
//! bones are emitted.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sanctum Prelate");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
