//! Sojourner's Enforcermite — `{6}` 6/6 Artifact Creature
//! (Frog Myr Salamander).
//!
//! Affinity for Affinity. (GAP — Affinity cost reduction is not in the
//! usable keyword surface for this class.)
//! Affinitycycling {2}. (A cycling-family variant; modeled as generic
//! Cycling {2} per the typecycling guidance — the search-for-an-affinity-
//! card payload is not separately modeled.)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sojourner's Enforcermite");
    let frog = reg.interner_mut().intern("Frog");
    let myr = reg.interner_mut().intern("Myr");
    let salamander = reg.interner_mut().intern("Salamander");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(frog);
    subtypes.0.insert(myr);
    subtypes.0.insert(salamander);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        // GAP: "Affinity for Affinity" cost-reduction static is not in the
        // usable keyword surface. Affinitycycling {2} modeled as generic
        // Cycling {2}.
        keywords: vec![KeywordAbility::Cycling(
            ManaCost::parse("{2}").expect("valid cost"),
        )],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
