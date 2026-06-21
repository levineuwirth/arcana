//! Hydradoodle — `{X}{X}{G}{G}` 0/0 Creature — Hydra Dog.
//! Reach, trample.
//! GAP: "As this creature enters, roll X six-sided dice. This creature enters
//! with a number of +1/+1 counters equal to the total of those results." —
//! there is no dice-roll primitive, and EntersWithSpec::CountersFromX would
//! place X counters (not the dice sum), which is materially different; the
//! enters-with clause is omitted rather than approximated.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hydradoodle");
    let hydra = reg.interner_mut().intern("Hydra");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);
    subtypes.0.insert(dog);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{X}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };
    reg.register(CardDefinition::new(name, chars))
}
