//! Revenant — `{4}{B}` */* Spirit with Flying.
//!
//! Oracle:
//! * Flying — base keyword.
//! * "Revenant's power and toughness are each equal to the number of
//!   creature cards in your graveyard." — GAP: a characteristic-
//!   defining static (P/T marked `*` via PtValue::Star); no
//!   triggered/activated form.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Revenant");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: CDA "power and toughness equal to the number of creature
    // cards in your graveyard" — continuous static (P/T marked `*`).
    reg.register(CardDefinition::new(name, chars))
}
