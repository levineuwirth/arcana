//! Dralnu's Pet — `{1}{U}{U}` 2/2 Creature — Shapeshifter.
//! Kicker—{2}{B}, Discard a creature card — not in the usable keyword surface
//! (and the discard-a-card kicker cost is non-mana); GAP'd.
//! "If this creature was kicked, it enters with flying and with X +1/+1 counters
//! on it, where X is the discarded card's mana value." — depends on the
//! cast-time kicked flag and the discarded card's mana value, neither of which
//! has a demonstrated accessor; GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dralnu's Pet");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Kicker—{2}{B}, Discard a creature card — non-mana kicker, not
        // in the usable keyword surface.
        ..Default::default()
    };

    // GAP: "If this creature was kicked, it enters with flying and with X +1/+1
    // counters on it, where X is the discarded card's mana value." — the kicked
    // flag and discarded-card mana value have no demonstrated trigger accessor.
    reg.register(CardDefinition::new(name, chars))
}
