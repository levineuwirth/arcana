//! B.F.M. (Big Furry Monster) — black Un-card creature. The spec provides no
//! mana cost or power/toughness, and the oracle text (two-card casting, "if one
//! leaves the battlefield sacrifice the other", "can't be blocked except by
//! three or more creatures") is not expressible with the available API.

use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("B.F.M. (Big Furry Monster)");
    let chars = Characteristics {
        name,
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        ..Default::default()
    };
    // GAP: "You must cast both B.F.M. cards" / "if one leaves, sacrifice the
    // other" / "can't be blocked except by three or more creatures" — none of
    // these multi-card / conditional-blocking statics are expressible. No mana
    // cost or power/toughness given in the spec.
    reg.register(CardDefinition::new(name, chars))
}
