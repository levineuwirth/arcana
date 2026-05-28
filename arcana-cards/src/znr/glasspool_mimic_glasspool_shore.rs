//! Glasspool Mimic // Glasspool Shore
//!
//! Front: Creature — Shapeshifter Rogue {2}{U} 0/0 (blue)
//!   You may have this creature enter as a copy of a creature you control, except it's a Shapeshifter Rogue in addition.
//! Back: Land (enters tapped; {T}: Add {U})
//! GAP: MDFC back face not modeled (mechanic deferred)
//! GAP: CopyPermanent-on-enter (enter-as-copy) is not modeled as an ETB trigger

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glasspool Mimic");
    let back_name = reg.interner_mut().intern("Glasspool Shore");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let rogue = reg.interner_mut().intern("Rogue");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(shapeshifter);
    subtypes.insert(rogue);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![],
        ..Default::default()
    };

    let back_chars = Characteristics {
        types: TypeLine::LAND.into(),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_mdfc_back(CardFace {
            name: back_name,
            characteristics: back_chars,
            spell_ability: None,
        }),
    )
}
