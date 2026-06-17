//! Stingerback Terror — `{2}{R}{R}` 7/7 Scorpion Dragon with Flying and Trample.
//! "This creature gets -1/-1 for each card in your hand.
//!  Plot {2}{R} (...)"
//!
//! Flying + Trample are base keywords. The static self-debuff "gets -1/-1 for
//! each card in your hand" is a continuous CDA with no primitive — GAP'd. Plot
//! is not a modeled keyword — GAP'd (not in the KeywordAbility set).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Stingerback Terror");
    let scorpion = reg.interner_mut().intern("Scorpion");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scorpion);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: static "gets -1/-1 for each card in your hand" — no continuous
    // self-debuff-per-hand-size primitive.
    // GAP: Plot {2}{R} — not a modeled keyword/alternate-cast mechanic.
    reg.register(CardDefinition::new(name, chars))
}
