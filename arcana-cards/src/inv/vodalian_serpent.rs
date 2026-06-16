//! Vodalian Serpent — `{3}{U}` 2/2 Serpent.
//! Kicker {2}. "This creature can't attack unless defending player
//! controls an Island." "If this creature was kicked, it enters with
//! four +1/+1 counters on it."
//!
//! Only the bones are wired. Kicker is not in the usable keyword surface
//! — GAP'd; the kicked-conditional ETB counters depend on that unmodeled
//! kicker state. The "can't attack unless defending player controls an
//! Island" attack restriction is a pure continuous static with no
//! triggered/activated form — GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vodalian Serpent");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(serpent);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Kicker {2} — not in the usable keyword surface.
        ..Default::default()
    };

    // GAP: "This creature can't attack unless defending player controls
    // an Island" — a pure continuous attack restriction static.
    // GAP: "If this creature was kicked, it enters with four +1/+1
    // counters on it" — depends on unmodeled kicked state.

    reg.register(CardDefinition::new(name, chars))
}
