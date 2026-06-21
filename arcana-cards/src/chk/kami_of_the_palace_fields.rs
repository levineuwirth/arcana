//! Kami of the Palace Fields — `{5}{W}` 3/2 Spirit.
//! Flying, first strike
//! Soulshift 5 (When this creature dies, you may return target Spirit card
//! with mana value 5 or less from your graveyard to your hand.)
//!
//! All three abilities are base/keyword characteristics: Flying and First
//! strike are evergreen keywords, and the engine synthesizes the Soulshift
//! death trigger from `KeywordAbility::Soulshift(5)`.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kami of the Palace Fields");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Flying,
            KeywordAbility::FirstStrike,
            KeywordAbility::Soulshift(5),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
