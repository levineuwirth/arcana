//! Hound of Urabrask — `{3}{R}{R}` 3/3 Phyrexian.
//!
//! Oracle:
//! * Oildying (When this dies, if it had no oil counters on it, return it to
//!   the battlefield under its owner's control with an oil counter on it).
//! * Hound of Urabrask gets +1/+1 for each oil counter on it.
//! * As long as Hound of Urabrask has an oil counter on it, it has double
//!   strike.
//!
//! Oildying is an Undying-variant with a named counter; it is not a usable
//! keyword and its "return from graveyard with an oil counter" payload has no
//! expressible Effect. The other two lines are self-referential static
//! continuous effects (dynamic pump and conditional keyword grant). All three
//! are GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hound of Urabrask");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: Oildying — Undying-variant keyword with a named (oil) counter; not a
    // usable keyword and no graveyard-return-with-counter primitive.
    // GAP: "gets +1/+1 for each oil counter on it" — self-referential dynamic
    // static pump; no primitive.
    // GAP: "as long as it has an oil counter, it has double strike" —
    // conditional static keyword grant; no primitive.
    reg.register(CardDefinition::new(name, chars))
}
