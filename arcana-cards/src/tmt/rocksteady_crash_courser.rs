//! Rocksteady, Crash Courser — `{4}{G}{G}` 7/7 legendary Rhino Mutant.
//! "Rocksteady can't be blocked by more than one creature." (static — GAP)
//! "Boars you control can't be blocked by more than one creature." (static — GAP)
//! "Forestcycling {2}" → modeled as the generic Cycling {2} keyword (the
//!  engine synthesizes "{2}, discard this card: draw a card"; the
//!  type-search variant is not separately modeled).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rocksteady, Crash Courser");
    let rhino = reg.interner_mut().intern("Rhino");
    let mutant = reg.interner_mut().intern("Mutant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(rhino);
    subtypes.0.insert(mutant);

    // GAP: "Rocksteady can't be blocked by more than one creature" and
    // "Boars you control can't be blocked by more than one creature" —
    // both are continuous block-restriction statics (no KeywordAbility /
    // Effect for "can't be blocked by more than one").
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
