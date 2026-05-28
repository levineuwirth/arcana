//! Esika, God of the Tree // The Prismatic Bridge
//!
//! Front: Legendary Creature — God {1}{G}{G} 1/4 (green)
//!   Vigilance; {T}: Add one mana of any color.
//!   Other legendary creatures you control have vigilance and "{T}: Add one mana of any color."
//! Back: Legendary Enchantment
//!   At the beginning of your upkeep, reveal cards from the top of your library until you reveal a creature or planeswalker card. Put that card onto the battlefield.
//! GAP: MDFC back face not modeled (mechanic deferred)
//! GAP: Static ability granting activated abilities to other permanents not modeled
//! GAP: Add-any-color mana ability not modeled (no AnyColor ManaColor variant exposed)

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Esika, God of the Tree");
    let back_name = reg.interner_mut().intern("The Prismatic Bridge");
    let god = reg.interner_mut().intern("God");

    let mut subtypes = SubtypeSet::new();
    subtypes.insert(god);

    let chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };

    let back_chars = Characteristics {
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
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
