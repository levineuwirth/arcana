//! Balustrade Wurm — `{3}{G}{G}` 5/5 Wurm with Trample and Haste.
//! This spell can't be countered (GAP — static uncounterable not expressible).
//! Delirium — {2}{G}{G}: Return this card from your graveyard to the
//! battlefield with a finality counter on it (GAP — see below).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Balustrade Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        // Delirium is not a usable keyword; Trample + Haste are.
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
        ..Default::default()
    };
    // GAP: "This spell can't be countered" — static uncounterability not
    // expressible.
    // GAP: "Delirium — {2}{G}{G}: Return this card from your graveyard to the
    // battlefield with a finality counter on it. Activate only if there are
    // four or more card types among cards in your graveyard and only as a
    // sorcery." A graveyard activated ability that returns its OWN source to
    // the battlefield (with a finality counter, gated by a graveyard
    // card-type count and sorcery-speed) is not expressible with the demon-
    // strated effect/cost primitives.
    reg.register(CardDefinition::new(name, chars))
}
