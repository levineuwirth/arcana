//! Enlisted Wurm — `{4}{G}{W}` 5/5 green/white Wurm.
//! Has Cascade (keyword not yet in engine API; best-effort stub).
//!
//! # Rules references
//!
//! * Cascade — When you cast this spell, exile cards from the top of your
//!   library until you exile a nonland card that costs less. You may cast it
//!   without paying its mana cost. Put the exiled cards on the bottom in a
//!   random order. Not expressible with current KeywordAbility variants;
//!   verify pipeline will flag for human routing.

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Enlisted Wurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
