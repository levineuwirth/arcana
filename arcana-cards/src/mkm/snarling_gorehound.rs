//! Snarling Gorehound — `{B}` 1/1 Dog (B). Menace.
//! "Whenever another creature you control with power 2 or less enters,
//! surveil 1."
//!
//! Menace is a base keyword. The ETB trigger is a filtered ZoneChange (a
//! creature you control with power ≤ 2 entering the battlefield) whose effect
//! is Surveil 1.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Snarling Gorehound");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Menace],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .with_max_power(2),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: surveil_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn surveil_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Surveil { player: trig.controller, count: 1 }]
}
