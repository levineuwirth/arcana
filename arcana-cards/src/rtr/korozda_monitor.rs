//! Korozda Monitor — `{2}{G}{G}` 3/3 Creature — Lizard with Trample and
//! Scavenge {5}{G}{G}.
//!
//! Trample.
//! Scavenge {5}{G}{G}.
//!
//! Both are base keywords. The engine synthesizes Scavenge's graveyard
//! activated ability from the `KeywordAbility::Scavenge(cost)` entry.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Korozda Monitor");
    let lizard = reg.interner_mut().intern("Lizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Scavenge(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
