//! Saheeli, Sublime Artificer — `{1}{U/R}{U/R}` Legendary Planeswalker —
//! Saheeli, starting loyalty 5. Colors R, U.
//!
//! Whenever you cast a noncreature spell, create a 1/1 colorless Servo
//! artifact creature token. (Modeled as a triggered ability.)
//! −2: Target artifact you control becomes a copy of another target
//!     artifact or creature you control until end of turn, except it's an
//!     artifact in addition to its other types. (GAP — copy effect.)

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Saheeli, Sublime Artificer");
    let saheeli = reg.interner_mut().intern("Saheeli");
    let _servo = reg.interner_mut().intern("Servo");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saheeli);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U/R}{U/R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: Some(ObjectFilter::permanent().without_types(TypeLine::CREATURE.into())),
                    caster: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: noncreature_cast_servo,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Target artifact you control becomes a copy of another target artifact or creature you control until end of turn, except it's an artifact in addition to its other types.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::permanent()
                                .with_types(TypeLine::ARTIFACT.into())
                                .controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter {
                            types_any: Some(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
                            controller: Some(ControllerConstraint::You),
                            ..Default::default()
                        }),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_copy,
            }),
    )
}

fn noncreature_cast_servo(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let servo = reg.interner().lookup("Servo").expect("Servo interned");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(servo);
    let token = TokenDefinition {
        name: servo,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken { controller: trig.controller, token }]
}

fn minus_two_copy(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a copy of another target artifact or creature you control
    //      until end of turn, except it's an artifact in addition" — the
    //      copy-another-permanent effect (CopyPermanent applies to a target,
    //      not "X becomes a copy of Y") with an added-type rider isn't
    //      expressible from the demonstrated surface.
    Vec::new()
}
