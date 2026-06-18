//! Exava, Rakdos Blood Witch — `{2}{B}{R}` 3/3 Legendary Human Cleric.
//! First strike, haste, unleash.
//! "Each other creature you control with a +1/+1 counter on it has haste."
//!
//! GAP: the static anthem-style grant ("each other creature you control with a
//! +1/+1 counter has haste") is a continuous effect with no triggered/activated
//! hook — not expressible here.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Exava, Rakdos Blood Witch");
    let human = reg.interner_mut().intern("Human");
    let cleric = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(cleric);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![
            KeywordAbility::FirstStrike,
            KeywordAbility::Haste,
            KeywordAbility::Unleash,
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
