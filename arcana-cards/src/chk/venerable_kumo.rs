//! Venerable Kumo — `{4}{G}` 2/3 Spirit with Reach and Soulshift 4.
//! "Soulshift 4 (When this creature dies, you may return target Spirit
//! card with mana value 4 or less from your graveyard to your hand.)"
//!
//! Both abilities are keyword-line entries: `KeywordAbility::Reach` and the
//! parametrized `KeywordAbility::Soulshift(4)`. The engine synthesizes the
//! Soulshift dies-trigger from the keyword, so no hand-rolled triggered
//! ability is needed.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Venerable Kumo");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Soulshift(4)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
