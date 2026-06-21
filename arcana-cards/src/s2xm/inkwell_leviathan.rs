//! Inkwell Leviathan — `{7}{U}{U}` 7/11 Artifact Creature — Leviathan.
//!
//! * Trample (keyword).
//! * Islandwalk → `KeywordAbility::Landwalk(intern("Island"))`.
//! * Shroud (keyword).
//!
//! Pure keyword line — no triggered or activated abilities.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inkwell Leviathan");
    let leviathan = reg.interner_mut().intern("Leviathan");
    let island = reg.interner_mut().intern("Island");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(leviathan);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(11)),
        keywords: vec![
            KeywordAbility::Trample,
            KeywordAbility::Landwalk(island),
            KeywordAbility::Shroud,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
