//! Lunar Hatchling — `{4}{G}{U}` 6/6 Creature — Alien Beast.
//! Flying, trample.
//! Basic landcycling {2} (modeled as generic Cycling {2} — the type-search
//! variant is not separately modeled).
//! Escape—{4}{G}{U}, ... (GAP — Escape is not in the usable keyword surface).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lunar Hatchling");
    let alien = reg.interner_mut().intern("Alien");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(alien);
    subtypes.0.insert(beast);

    // GAP: Escape—{4}{G}{U}, Exile a land, Exile five other cards from your
    // graveyard — Escape is not an available keyword for this card class.

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Trample,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
