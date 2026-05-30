//! Jetfire, Ingenious Scientist // Jetfire, Air Guardian
//!
//! Front: {4}{U} Legendary Artifact Creature — Robot 3/4.
//! GAP: More Than Meets the Eye {3}{U} alternate cast cost not modeled.
//! Flying.
//! GAP: "Remove one or more +1/+1 counters from among artifacts you control: Target player
//! adds that much {C}. This mana can't be spent to cast nonartifact spells. Convert Jetfire."
//! — variable counter removal, restricted colorless mana, and Convert are not modeled.
//!
//! Back: Legendary Artifact — Vehicle.
//! GAP: Living metal not modeled.
//! Flying.
//! GAP: {U}{U}{U}: Convert Jetfire, then adapt 3. — Adapt not in Effect catalog; Convert not modeled.
//! GAP: back-face-only activated ability not modeled.

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jetfire, Ingenious Scientist");
    let sub_robot = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub_robot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Jetfire, Air Guardian");
    let sub_vehicle = reg.interner_mut().intern("Vehicle");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(sub_vehicle);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine(TypeLine::ARTIFACT),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    // GAP: More Than Meets the Eye alternate cast cost not modeled.
    // GAP: Front-face activated ability (remove +1/+1 counters → add {C} + Convert) not modeled:
    //   variable counter removal from artifacts, restricted mana, and Convert are engine gaps.
    // GAP: back-face-only activated ability ({U}{U}{U}: Convert + adapt 3) not modeled.
    reg.register(CardDefinition::new(name, chars).with_transform_back(back))
}
