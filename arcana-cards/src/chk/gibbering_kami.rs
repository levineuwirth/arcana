//! Gibbering Kami — `{3}{B}` 2/2 Spirit with Flying and Soulshift 3.
//! "Soulshift 3 (When this creature dies, you may return target Spirit
//! card with mana value 3 or less from your graveyard to your hand.)"
//!
//! Both are engine keyword abilities — Flying evergreen, Soulshift
//! parametrized — so the card is just the keyword line; the engine
//! synthesizes the Soulshift death trigger.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gibbering Kami");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Soulshift(3)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
