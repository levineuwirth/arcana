//! Golbez, Crystal Collector — `{U}{B}` 1/4 Legendary Creature — Human Wizard.
//!
//! * Whenever an artifact you control enters, surveil 1.
//! * At the beginning of your end step, if you control four or more artifacts,
//!   return target creature card from your graveyard to your hand. Then if you
//!   control eight or more artifacts, each opponent loses life equal to that
//!   card's power.
//!
//! (Surveil is not in the usable KeywordAbility surface — its rules are
//! captured by the triggered ability below; `keywords` is empty.)

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Golbez, Crystal Collector");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "Whenever an artifact you control enters, surveil 1."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: surveil_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // "At the beginning of your end step, if you control four or more
            // artifacts, return target creature card from your graveyard to
            // your hand. Then if you control eight or more artifacts, each
            // opponent loses life equal to that card's power."
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_four_artifacts),
                effect: end_step_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn surveil_one(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Surveil {
        player: trig.controller,
        count: 1,
    }]
}

fn if_four_artifacts(s: &GameState, _src: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::you_control_at_least(
        s,
        you,
        &ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        4,
    )
}

fn end_step_reanimate(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    // Capture the chosen card's power BEFORE it leaves the graveyard.
    let card_power = script::power_of(state, *id).max(0) as u32;

    let mut effects = vec![Effect::ReturnFromGraveyardToHand { target: *id }];

    // "Then if you control eight or more artifacts, each opponent loses life
    // equal to that card's power."
    let eight_artifacts = conditions::you_control_at_least(
        state,
        trig.controller,
        &ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
        8,
    );
    if eight_artifacts && card_power > 0 {
        for opp in script::opponents(state, trig.controller) {
            effects.push(Effect::LoseLife {
                player: opp,
                amount: card_power,
            });
        }
    }
    effects
}
