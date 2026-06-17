//! Illusionary Wall — `{4}{U}` 7/4 Illusion Wall with Defender, Flying, First strike.
//! Cumulative upkeep {U}.
//!
//! Cumulative upkeep is not in the usable keyword surface (no `KeywordAbility`
//! variant, and the per-age-counter upkeep tax is not expressible) — GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Illusionary Wall");
    let illusion = reg.interner_mut().intern("Illusion");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(illusion);
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Defender,
            KeywordAbility::Flying,
            KeywordAbility::FirstStrike,
        ],
        ..Default::default()
    };

    // GAP: Cumulative upkeep {U} — not in the usable keyword surface.
    reg.register(CardDefinition::new(name, chars))
}
