//! Arcbound Slasher — `{4}{R}` 0/0 Artifact Creature — Cat.
//! Modular 4, Riot. Both are engine-wired keywords (Modular(4) enters with four
//! +1/+1 counters and the dies-trigger; Riot adds the enters-with-counter-or-
//! haste choice).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcbound Slasher");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Modular(4), KeywordAbility::Riot],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
