//! Torii Watchward — `{4}{W}` 3/3 Spirit with Vigilance and Soulshift 4.
//!
//! Oracle:
//!  * Vigilance.
//!  * Soulshift 4 (When this creature dies, you may return target Spirit card
//!    with mana value 4 or less from your graveyard to your hand.)
//!
//! Both are keyword abilities; the engine synthesizes the Soulshift dies-trigger
//! from the keyword. Nothing beyond listing them in `keywords` is required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Torii Watchward");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Soulshift(4)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
