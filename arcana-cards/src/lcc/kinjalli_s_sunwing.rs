//! Kinjalli's Sunwing — `{2}{W}` 2/3 Dinosaur with Flying.
//! Flying.
//! Creatures your opponents control enter tapped.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kinjalli's Sunwing");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    // GAP: "Creatures your opponents control enter tapped." is a static
    // replacement effect (CR 614) — not a triggered or activated ability,
    // and there is no Effect/keyword to express a board-wide enters-tapped
    // replacement here.
    reg.register(CardDefinition::new(name, chars))
}
