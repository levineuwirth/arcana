//! Oathsworn Knight — `{1}{B}{B}` 0/0 Human Knight.
//!
//! Oracle:
//! * This creature enters with four +1/+1 counters on it.
//! * This creature attacks each combat if able.
//! * If damage would be dealt to this creature while it has a +1/+1 counter on
//!   it, prevent that damage and remove a +1/+1 counter from it.
//!
//! All three lines are GAP'd:
//! * "enters with four +1/+1 counters" is an as-enters replacement with no
//!   expressible hook in this card class (no enters-with constructor in the
//!   demonstrated surface).
//! * "attacks each combat if able" is a static attack requirement with no
//!   expressible primitive.
//! * The damage-prevention / remove-counter clause is a replacement effect,
//!   not a triggered/activated ability.
//!
//! Bones only.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oathsworn Knight");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
