//! Rainbow Crow — `{3}{U}` 2/2 blue Bird with Flying.
//!
//! * Flying (keyword).
//! * `{1}: This creature becomes the color of your choice until end of
//!   turn.` — GAP: `Effect::SetColor` requires a fixed `colors` value;
//!   there is no player-choice-of-color primitive in the usable surface,
//!   so the chosen-color activated ability is omitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rainbow Crow");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "{1}: This creature becomes the color of your choice until end
    // of turn." — no player-chosen-color activated effect is expressible.

    reg.register(CardDefinition::new(name, chars))
}
