//! Control Win Condition — `{4}{U}{U}` */* blue Whale.
//!
//! Oracle:
//! * "This spell can't be countered." (static cast restriction — GAP)
//! * Shroud
//! * "Control Win Condition's power and toughness are each equal to
//!   the number of turns you've taken this game." (CDA — GAP)
//!
//! Base power/toughness are `*` (`PtValue::Star`). The
//! characteristic-defining ability that sets `*` to the number of turns
//! you've taken this game is not computable: the engine tracks only a
//! global `turn.turn_number`, with no PER-PLAYER "turns you've taken"
//! counter, and extra turns make `turn_number` diverge from a single
//! player's turn count. No script helper exposes it either, so the CDA
//! is GAP'd; only Shroud is wired.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Control Win Condition");
    let whale = reg.interner_mut().intern("Whale");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(whale);

    // GAP: "This spell can't be countered." — no can't-be-countered
    // static primitive in the demonstrated API.
    // GAP: "power and toughness each equal to the number of turns
    // you've taken this game" — no per-player turns-taken counter
    // (only global turn_number); CDA not computable; base P/T left as *.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Shroud],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
