//! Somber Hoverguard — `{5}{U}` 3/2 Drone with Flying.
//! Affinity for artifacts (cost reduction — GAP; not an expressible keyword).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Somber Hoverguard");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drone);

    // GAP: "Affinity for artifacts" (cost reduction) is not an expressible
    // KeywordAbility for this card class — keywords carries only Flying.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
