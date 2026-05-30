//! Arcee, Sharpshooter // Arcee, Acrobatic Coupe
//!
//! Front: {1}{R}{W} Legendary Artifact Creature — Robot 2/2
//! First strike.
//! {1}, Remove one or more +1/+1 counters from Arcee: It deals that much damage
//! to target creature. Convert Arcee.
//! GAP: "More Than Meets the Eye" alternate cast cost not modeled.
//! GAP: "Convert Arcee" (the vehicle conversion mechanic) is not modeled.
//! GAP: "Living metal" keyword not in engine keyword surface.
//! GAP: activated ability "{1}, Remove one or more +1/+1 counters: deal that
//! much damage to target creature, Convert Arcee" — variable counter-removal
//! cost + Convert not expressible; omitted.
//! GAP: front-face trigger "Whenever you cast a spell that targets one or more
//! creatures or Vehicles you control, put that many +1/+1 counters on Arcee.
//! Convert Arcee." — "targets one or more creatures or Vehicles you control"
//! trigger condition not available; omitted.
//!
//! Back: Legendary Artifact — Vehicle (no P/T when not a creature).

use arcana_core::effects::KeywordAbility;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Arcee, Sharpshooter");
    let robot_sub = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::FirstStrike],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Arcee, Acrobatic Coupe");
    let vehicle_sub = reg.interner_mut().intern("Vehicle");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vehicle_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::white(),
            types: TypeLine::ARTIFACT.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            // GAP: Living metal grants creature-hood during controller's turn;
            // back face P/T when active as creature not modeled.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
    )
}
