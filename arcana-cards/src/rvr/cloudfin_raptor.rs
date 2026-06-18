//! Cloudfin Raptor — `{U}` 0/1 Bird Mutant with Flying and Evolve.
//! "Evolve (Whenever a creature you control enters, if that creature
//! has greater power or toughness than this creature, put a +1/+1
//! counter on this creature.)"
//!
//! Both keywords are base characteristics; Evolve is a fully
//! implemented parametrized-marker keyword, so listing it in
//! `keywords` wires the engine's evolve trigger.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cloudfin Raptor");
    let bird = reg.interner_mut().intern("Bird");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);
    subtypes.0.insert(mutant);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Evolve],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
