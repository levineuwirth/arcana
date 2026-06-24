//! Sutured Ghoul — `{4}{B}{B}{B}` */* Zombie with Trample.
//! As this creature enters, exile any number of creature cards from your
//! graveyard. Its power/toughness equal the total power/toughness of the
//! exiled cards.
//!
//! Trample is expressible. The ETB exile-and-set-P/T characteristic-
//! defining ability is NOT expressible with the available self-CDA
//! constructors: `*` is the SUM of the powers (resp. toughnesses) of a
//! player-chosen set of creature cards exiled from the graveyard as this
//! enters. `self_pt_from_match` counts matching battlefield permanents and
//! `self_pt_cda` reads only state scalars — neither can capture the summed
//! P/T of a specific exiled-card set, so P/T are left as `*` and the ETB is
//! GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sutured Ghoul");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: "As this creature enters, exile any number of creature cards
    // from your graveyard; its P/T equal the total power/toughness of
    // the exiled cards" — an as-enters exile-and-define-P/T CDA is not
    // expressible (no primitive sets base P/T from an exiled-cards sum).
    reg.register(CardDefinition::new(name, chars))
}
