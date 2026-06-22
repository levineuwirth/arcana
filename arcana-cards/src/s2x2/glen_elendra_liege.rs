//! Glen Elendra Liege — `{1}{U/B}{U/B}{U/B}` 2/3 Faerie Knight (black/blue).
//! Flying.
//! Other blue creatures you control get +1/+1.
//! Other black creatures you control get +1/+1.
//!
//! Flying is a base keyword. The two anthem clauses are pure STATIC
//! continuous abilities (no trigger word, no activation cost) and are
//! not expressible in the MultiAbilityCreature shape — GAP'd below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glen Elendra Liege");
    let faerie = reg.interner_mut().intern("Faerie");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U/B}{U/B}{U/B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: static "Other blue creatures you control get +1/+1." (continuous anthem, not a triggered/activated ability).
    // GAP: static "Other black creatures you control get +1/+1." (continuous anthem, not a triggered/activated ability).
    reg.register(CardDefinition::new(name, chars))
}
