//! Sire of Seven Deaths — `{7}` 7/7 colorless Eldrazi.
//! First strike, vigilance, menace, trample, reach, lifelink.
//! Ward—Pay 7 life (non-mana ward — not expressible).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sire of Seven Deaths");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        keywords: vec![
            KeywordAbility::FirstStrike,
            KeywordAbility::Vigilance,
            KeywordAbility::Menace,
            KeywordAbility::Trample,
            KeywordAbility::Reach,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };
    // GAP: "Ward—Pay 7 life" — a non-mana ward cost; only mana ward (KeywordAbility::Ward(ManaCost))
    // is expressible, so this ward is omitted.
    reg.register(CardDefinition::new(name, chars))
}
