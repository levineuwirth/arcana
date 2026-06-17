//! Tabaxi Toucaneers — `{4}{W}` 3/2 Cat Ranger with Flying.
//! Flying
//! Myriad (token-copy-per-opponent attack ability) — GAP'd: Myriad is not
//!  in the keyword surface and the per-opponent attacking-copy machinery has
//!  no expressible primitive.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tabaxi Toucaneers");
    let cat = reg.interner_mut().intern("Cat");
    let ranger = reg.interner_mut().intern("Ranger");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(ranger);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
