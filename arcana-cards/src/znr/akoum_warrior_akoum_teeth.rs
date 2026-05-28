//! Akoum Warrior // Akoum Teeth
//!
//! Front: Creature — Minotaur Warrior {5}{R} 4/5 (red)
//!   Trample
//! Back: Land (enters tapped; {T}: Add {R})
//! GAP: MDFC back face not modeled (mechanic deferred)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Akoum Warrior");
    let back_name = reg.interner_mut().intern("Akoum Teeth");
    let minotaur = reg.interner_mut().intern("Minotaur");
    let warrior = reg.interner_mut().intern("Warrior");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(minotaur);
    subtypes.insert(warrior);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
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
