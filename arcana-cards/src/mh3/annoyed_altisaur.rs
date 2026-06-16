//! Annoyed Altisaur — `{5}{G}{G}` 6/5 Creature — Dinosaur.
//! Reach, trample.
//! Cascade.
//!
//! Bones + Reach and Trample are faithful. Cascade is a cast-trigger ("when you
//! cast this spell, …") that fires on THIS card's own cast; the usable
//! TriggerCondition catalog has no self-cast variant to host the Effect::Cascade
//! body, and Cascade is not among the usable KeywordAbility variants — so it is
//! GAP'd (Ethersworn Sphinx precedent).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Annoyed Altisaur");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Reach, KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: Cascade — fires on this spell's own cast; no self-cast
    //      TriggerCondition is available to host the Effect::Cascade body, and
    //      Cascade is not a usable KeywordAbility variant.
    reg.register(CardDefinition::new(name, chars))
}
