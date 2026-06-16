//! Body of Jukai — `{7}{G}{G}` 8/5 green Spirit.
//! Trample.
//! Soulshift 8 (When this creature dies, you may return target Spirit card
//! with mana value 8 or less from your graveyard to your hand.)
//!
//! Both abilities are base keywords: Trample is evergreen combat, and
//! Soulshift 8 is the engine-synthesized dies-return — listing them in
//! `keywords` is all that is required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Body of Jukai");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Soulshift(8)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
