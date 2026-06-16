//! Thundering Mightmare — `{4}{G}` 3/3 Horse Spirit.
//!
//! Soulbond (not a supported KeywordAbility variant — GAP'd.)
//! "As long as Thundering Mightmare is paired with another creature, each
//!  of those creatures has 'Whenever an opponent casts a spell, put a
//!  +1/+1 counter on this creature.'" (conditional static ability grant
//!  contingent on soulbond pairing — not expressible; GAP'd.)

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thundering Mightmare");
    let horse = reg.interner_mut().intern("Horse");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
