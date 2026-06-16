//! Horde of Notions — `{W}{U}{B}{R}{G}` 5/5 Legendary Elemental with
//! Vigilance, Trample, and Haste.
//! "{W}{U}{B}{R}{G}: You may play target Elemental card from your graveyard
//! without paying its mana cost."

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Horde of Notions");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue() | ColorSet::black() | ColorSet::red() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
            KeywordAbility::Haste,
        ],
        ..Default::default()
    };

    // GAP: "{W}{U}{B}{R}{G}: You may play target Elemental card from your
    // graveyard without paying its mana cost" — no Effect primitive for
    // casting/playing a targeted graveyard card for free (the reanimation
    // primitives put a permanent directly onto the battlefield, which is not
    // the same as "play without paying" and would mishandle noncreature
    // Elementals).
    reg.register(CardDefinition::new(name, chars))
}
