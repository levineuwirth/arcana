//! Oona's Gatewarden — `{U/B}` 2/1 black/blue Faerie Soldier.
//!
//! * Defender, Flying, Wither — all keyword-line abilities (no triggered or
//!   activated abilities on this card).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oona's Gatewarden");
    let faerie = reg.interner_mut().intern("Faerie");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(faerie);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U/B}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![
            KeywordAbility::Defender,
            KeywordAbility::Flying,
            KeywordAbility::Wither,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
