//! Apex Hawks — `{2}{W}` 2/2 Bird with Flying.
//! Multikicker {1}{W} and "enters with a +1/+1 counter for each time it
//! was kicked" — kicker is unmodeled, so only Flying is expressible.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Apex Hawks");
    let bird = reg.interner_mut().intern("Bird");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bird);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Multikicker keyword — not in the available KeywordAbility surface.
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "enters with a +1/+1 counter on it for each time it was kicked" —
    // kicker / multikicker payment count is not tracked by the engine.
    reg.register(CardDefinition::new(name, chars))
}
