//! Sluiceway Scorpion — `{2}{B}{G}` 2/2 Creature — Scorpion (B/G).
//!
//! * Deathtouch — base keyword.
//! * Scavenge `{1}{B}{G}` — parametrized keyword; the engine synthesizes the
//!   graveyard activated ability automatically.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sluiceway Scorpion");
    let scorpion = reg.interner_mut().intern("Scorpion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scorpion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Deathtouch,
            KeywordAbility::Scavenge(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
