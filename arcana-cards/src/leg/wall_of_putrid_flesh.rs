//! Wall of Putrid Flesh — `{2}{B}` 2/4 Wall with Defender.
//! "Protection from white" — GAP (Protection keyword not modeled).
//! "Prevent all damage that would be dealt to this creature by
//! enchanted creatures" — GAP (static replacement, no trigger/cost).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wall of Putrid Flesh");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    // GAP: "Protection from white" — Protection is not an available
    // KeywordAbility variant for this card class.
    // GAP: "Prevent all damage that would be dealt to this creature by
    // enchanted creatures" — pure static replacement, no triggered or
    // activated ability to attach.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Defender],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
