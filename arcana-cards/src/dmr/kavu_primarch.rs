//! Kavu Primarch — `{3}{G}` 3/3 Kavu. Kicker {4}, Convoke.
//! "If this creature was kicked, it enters with four +1/+1 counters on it."
//! Kicker and Convoke are not in the usable KeywordAbility surface, and the
//! kicked-enters-with-counters replacement is GAP'd.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kavu Primarch");
    let kavu = reg.interner_mut().intern("Kavu");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kavu);

    // GAP: Kicker {4} — not a usable KeywordAbility variant.
    // GAP: Convoke — not a usable KeywordAbility variant.
    // GAP: "If this creature was kicked, it enters with four +1/+1 counters on
    // it." — a kicker-gated enters-with-counters replacement, not expressible.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
