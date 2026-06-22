//! Citadel Castellan — `{1}{G}{W}` 2/3 Human Knight (green/white).
//! Vigilance, Renown 2.
//!
//! Both lines are keywords. Renown 2's reminder text ("When this creature deals
//! combat damage to a player, if it isn't renowned, put two +1/+1 counters on it
//! and it becomes renowned") is the keyword's built-in behavior — nothing beyond
//! listing the keywords is required.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Citadel Castellan");
    let human = reg.interner_mut().intern("Human");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(knight);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Vigilance, KeywordAbility::Renown(2)],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
