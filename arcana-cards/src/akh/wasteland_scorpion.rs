//! Wasteland Scorpion — `{2}{B}` 2/2 Scorpion with Deathtouch and Cycling {2}.
//!
//! Deathtouch is a base keyword; Cycling {2} is the parametrized keyword
//! (the engine synthesizes the "[cost], discard this card: draw a card"
//! activated ability from it).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wasteland Scorpion");
    let scorpion = reg.interner_mut().intern("Scorpion");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scorpion);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![
            KeywordAbility::Deathtouch,
            KeywordAbility::Cycling(ManaCost::parse("{2}").expect("valid cost")),
        ],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars))
}
