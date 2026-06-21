//! Altered Ego — `{X}{2}{G}{U}` 0/0 Creature — Shapeshifter.
//!
//! * This spell can't be countered. (Casting static — not expressible;
//!   GAP'd.)
//! * You may have this creature enter as a copy of any creature on the
//!   battlefield, except it enters with X additional +1/+1 counters on it.
//!   (Enter-as-a-copy replacement — not expressible; GAP'd.)
//!
//! GAP: "can't be countered" is a casting-time static with no primitive.
//! GAP: the Clone-style "enter as a copy of any creature, plus X +1/+1
//! counters" replacement has no expressible primitive (CopyPermanent mints a
//! new token rather than altering how this object enters).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Altered Ego");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
