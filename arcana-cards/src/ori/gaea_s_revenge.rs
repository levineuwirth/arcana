//! Gaea's Revenge — `{5}{G}{G}` 8/5 Elemental with Haste.
//! "This spell can't be countered."
//! "Haste"
//! "This creature can't be the target of nongreen spells or abilities
//! from nongreen sources."
//!
//! Haste is wired. The "can't be countered" cast-static and the
//! conditional hexproof-from-nongreen static are not expressible with the
//! usable surface and are GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

// GAP: "This spell can't be countered." — an uncounterable cast-static
// with no expressible representation in this card class.
// GAP: "This creature can't be the target of nongreen spells or abilities
// from nongreen sources." — color-conditional hexproof; no Effect /
// KeywordAbility expresses source-color-restricted untargetability.

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gaea's Revenge");
    let elemental = reg.interner_mut().intern("Elemental");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
