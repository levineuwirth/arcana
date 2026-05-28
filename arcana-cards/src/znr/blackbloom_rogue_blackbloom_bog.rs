//! Blackbloom Rogue // Blackbloom Bog
//!
//! Front: Creature — Human Rogue {2}{B} 2/3 (black)
//!   Menace
//!   This creature gets +3/+0 as long as an opponent has eight or more cards in their graveyard.
//! Back: Land (enters tapped; {T}: Add {B})
//! GAP: MDFC back face not modeled (mechanic deferred)
//! GAP: Conditional static P/T boost based on opponent's graveyard size not modeled

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blackbloom Rogue");
    let back_name = reg.interner_mut().intern("Blackbloom Bog");
    let human = reg.interner_mut().intern("Human");
    let rogue = reg.interner_mut().intern("Rogue");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(human);
    subtypes.insert(rogue);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
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
