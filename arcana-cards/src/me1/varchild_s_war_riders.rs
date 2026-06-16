//! Varchild's War-Riders — `{1}{R}` 3/4 Human Warrior with Trample and Rampage 1.
//! "Cumulative upkeep—Have an opponent create a 1/1 red Survivor token." (GAP).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Varchild's War-Riders");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample, KeywordAbility::Rampage(1)],
        ..Default::default()
    };

    // GAP: "Cumulative upkeep—Have an opponent create a 1/1 red Survivor token" —
    // Cumulative upkeep (age-counter pay-per-counter mechanic) is not in the usable
    // keyword surface and has no expressible trigger/cost form.
    reg.register(CardDefinition::new(name, chars))
}
