//! Blaster, Combat DJ // Blaster, Morale Booster
//!
//! Front face: {3}{R}{G} Legendary Artifact Creature — Robot 3/3
//! More Than Meets the Eye {1}{R}{G} (cast converted for {1}{R}{G}).
//! Other nontoken artifact creatures and Vehicles you control have modular 1.
//! Whenever you put one or more +1/+1 counters on Blaster, convert it.
//!
//! Back face: Legendary Artifact
//! Modular 3.
//! {X},{T}: Move X +1/+1 counters from Blaster onto another target artifact.
//! That artifact gains haste until end of turn. If Blaster has no +1/+1 counters, convert it.
//! Activate only as a sorcery.
//!
//! GAP: More Than Meets the Eye alternate-cast cost not modeled (transform-specific cast variant).
//! GAP: "Other nontoken artifact creatures and Vehicles you control have modular 1" — static
//!   grant-to-others layer not modeled by the engine.
//! GAP: Triggered "whenever you put one or more +1/+1 counters on Blaster, convert it" —
//!   TriggerCondition::CounterAdded fires per add-event but does not gate on "one or more" batch;
//!   modeled as front-face triggered ability that calls Transform.
//! GAP: Back-face activated ability ({X},{T}: move counters, haste, conditional convert) —
//!   activated-ability X-cost and counter-move are not modeled; back-face-only ability not installed.
//! GAP: back-face-only triggered ability not modeled.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blaster, Combat DJ");
    let robot_sub = reg.interner_mut().intern("Robot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(robot_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{G}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::green(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: front face has no modular on itself; "other creatures have modular 1" is a
        //   static grant-to-others effect not modeled by keywords on this card.
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Blaster, Morale Booster");
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red() | ColorSet::green(),
            types: TypeLine(TypeLine::ARTIFACT),
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![KeywordAbility::Modular(3)],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face trigger: whenever +1/+1 counters are added to Blaster, convert it.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::PlusOnePlusOne),
                    chapter: None,
                },
                intervening_if: None,
                effect: on_counter_added_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_counter_added_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
