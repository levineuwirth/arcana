//! Vorapede — `{2}{G}{G}{G}` 5/4 Insect with Vigilance, Trample, and
//! Undying.
//!
//! All three are keyword characteristics: Vigilance and Trample are
//! evergreen combat keywords, and Undying ("When this creature dies,
//! if it had no +1/+1 counters on it, return it to the battlefield
//! under its owner's control with a +1/+1 counter on it") is the
//! engine-implemented `KeywordAbility::Undying` variant — its
//! death-trigger machinery is synthesized from the keyword, so nothing
//! beyond listing it is required. There are no further triggered or
//! activated abilities to decompose.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vorapede");
    let insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Trample,
            KeywordAbility::Undying,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
