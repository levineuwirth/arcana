//! Kunoros, Hound of Athreos — `{1}{W}{B}` 3/3 Legendary Dog.
//! Vigilance, menace, lifelink. Two static rule-altering abilities ("Creature
//! cards in graveyards can't enter the battlefield" / "Players can't cast spells
//! from graveyards") are global statics with no triggered/activated expression —
//! GAP'd. Keyword line is emitted.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kunoros, Hound of Athreos");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::Vigilance,
            KeywordAbility::Menace,
            KeywordAbility::Lifelink,
        ],
        ..Default::default()
    };

    // GAP: static "Creature cards in graveyards can't enter the battlefield".
    // GAP: static "Players can't cast spells from graveyards".
    reg.register(CardDefinition::new(name, chars))
}
