//! Sojourner's Companion — `{7}` 4/4 Artifact Creature — Salamander.
//! Affinity for artifacts (GAP — not a usable KeywordAbility for this
//! class). Artifact landcycling {2}.
//!
//! Per cycling conventions, the typecycling/landcycling variant maps to
//! generic `KeywordAbility::Cycling` with its printed cost {2}; the
//! library-search-for-an-artifact-land specialization is not separately
//! modeled (a documented partial). Affinity for artifacts (the cost
//! reduction) is not expressible — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sojourner's Companion");
    let salamander = reg.interner_mut().intern("Salamander");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(salamander);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        // GAP: "Affinity for artifacts" (cost reduction) is not a usable
        // KeywordAbility variant for this class.
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
