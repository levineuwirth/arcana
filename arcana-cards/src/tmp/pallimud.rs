//! Pallimud — `{2}{R}` */3 Beast.
//! As this creature enters, choose an opponent.
//! "Pallimud's power is equal to the number of tapped lands the chosen
//! player controls." (a CDA tied to an as-enters choice — GAP'd.)
//!
//! No keyword line. The power is `*` (PtValue::Star) and the toughness is
//! a fixed 3. The "as enters, choose an opponent" replacement choice and
//! the chosen-player tapped-lands CDA are not expressible via the
//! demonstrated API (no as-enters choice slot and no CDA installer), so
//! both are GAP'd; PtValue::Star marks the defining-static slot.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pallimud");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "As this creature enters, choose an opponent" — no as-enters
    // choice slot.
    // GAP (CDA): "Pallimud's power is equal to the number of tapped lands the
    // chosen player controls" — no CDA installer; PtValue::Star marks the slot.

    reg.register(CardDefinition::new(name, chars))
}
