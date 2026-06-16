//! Boarding Party — `{5}{R}` 6/3 Human Pirate with Haste.
//!
//! Oracle:
//! * Haste.
//! * Cascade.
//!
//! Haste is a base characteristic. Cascade is not in the usable keyword
//! surface for this card class and there is no listed self-cast trigger
//! condition on which to hang `Effect::Cascade`, so it is GAP'd.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boarding Party");
    let human = reg.interner_mut().intern("Human");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pirate);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    // GAP: Cascade — not in the usable keyword surface and no listed
    // self-cast trigger condition to drive Effect::Cascade.
    reg.register(CardDefinition::new(name, chars))
}
