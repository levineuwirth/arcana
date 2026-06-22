//! Zanikev Locust — `{5}{B}` 3/3 Creature — Insect with Flying and Scavenge {2}{B}{B}.
//! "Flying
//!  Scavenge {2}{B}{B} ({2}{B}{B}, Exile this card from your graveyard: Put a number of
//!  +1/+1 counters equal to this card's power on target creature. Scavenge only as a
//!  sorcery.)"
//!
//! Both abilities are keywords; the engine synthesizes the Scavenge graveyard ability.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Zanikev Locust");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::Scavenge(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
