//! Voidpouncer — `{1}{R}` 3/1 Eldrazi.
//! "Devoid (This card has no color.)"
//! "Kicker {2}{C}"
//! "If this creature was kicked, it enters with two +1/+1 counters and
//!  a trample counter on it and with haste."
//!
//! Bones only. Devoid is modeled as colorless. GAP: Kicker is not a
//! usable KeywordAbility variant and there is no kicker cost / kicked-
//! ETB-replacement primitive, so the "if kicked" enter-modification
//! (counters + trample counter + haste) cannot be expressed.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Voidpouncer");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "Kicker {2}{C}" + "If this creature was kicked, it enters
    // with two +1/+1 counters, a trample counter, and haste." — no
    // kicker cost field, no kicked-ETB replacement primitive.

    reg.register(CardDefinition::new(name, chars))
}
