//! Sulam Djinn — `{5}{G}` 6/6 green Djinn with Trample.
//! "This creature gets -2/-2 as long as green is the most common color
//! among all permanents or is tied for most common."
//!
//! Decomposition:
//! * keyword line → `KeywordAbility::Trample`.
//! * The -2/-2 clause is a STATIC continuous ability (no trigger word,
//!   no cost) whose condition ("green is the most common color among
//!   all permanents or tied for most common") has no expressible
//!   primitive in the demonstrated API. See GAP below.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: static "gets -2/-2 as long as green is the most common color among
// all permanents or is tied for most common" — a conditional continuous
// self-debuff gated on a board-wide most-common-color computation. No
// triggered/activated ability and no static-effect / color-census
// primitive is available in the demonstrated API. Omitted.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sulam Djinn");
    let djinn = reg.interner_mut().intern("Djinn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(djinn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
