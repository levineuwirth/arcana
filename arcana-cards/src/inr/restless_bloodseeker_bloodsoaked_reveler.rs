//! Restless Bloodseeker // Bloodsoaked Reveler
//!
//! Front: Creature — Vampire {1}{B}, 1/3
//! At the beginning of your end step, if you gained life this turn, create a Blood token.
//! Sacrifice two Blood tokens: Transform this creature. Activate only as a sorcery.
//! (GAP: "Sacrifice two Blood tokens" as an activated ability cost not modeled —
//!  SacrificeOther cost not in OptionalPaymentKind. The transform is not triggered here.)
//! ("if you gained life this turn" wired as intervening_if via
//!  conditions::you_gained_life_this_turn.)
//!
//! Back (Bloodsoaked Reveler): Creature — Vampire
//! At the beginning of your end step, if you gained life this turn, create a Blood token.
//! {4}{B}: Each opponent loses 2 life and you gain 2 life.
//! The back-face end-step Blood trigger is wired (id 2, gated to face 1) and the
//! back-face activated ability {4}{B} is wired via face_gate: Some(1). The front
//! end-step trigger is gated to face 0.

use arcana_core::conditions;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::script;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::ControllerConstraint;
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Restless Bloodseeker");

    let vampire = reg.interner_mut().intern("Vampire");
    let mut front_subtypes = SubtypeSet::default();
    front_subtypes.0.insert(vampire);

    // Pre-intern Blood subtype for resolver use
    let _blood = reg.interner_mut().intern("Blood");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes: front_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![],
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Bloodsoaked Reveler");
    let vampire2 = reg.interner_mut().intern("Vampire");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(vampire2);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front-face trigger: at beginning of your end step, if you gained life this turn,
            // create a Blood token. Intervening-if wired via
            // conditions::you_gained_life_this_turn.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(iif_gained_life),
                effect: end_step_blood_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face (Bloodsoaked Reveler): same end-step Blood-token trigger.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(iif_gained_life),
                effect: end_step_blood_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: {4}{B}: Each opponent loses 2 life and you gain 2 life.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{B}: Each opponent loses 2 life and you gain 2 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1),
                effect: drain_each_opponent,
            })
            // Front end-step trigger only on the front face; the back end-step
            // trigger only on the back face.
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 1)
            // GAP: the front-face "Sacrifice two Blood tokens: Transform this creature"
            // activated ability is not wired — ActivationCost has no "sacrifice N
            // tokens matching a filter" cost (no SacrificeOther-by-filter cost kind).
    )
}

/// Back face: each opponent loses 2 life and you gain 2 life.
fn drain_each_opponent(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, ctx.controller)
        .into_iter()
        .map(|p| Effect::LoseLife { player: p, amount: 2 })
        .collect();
    effects.push(Effect::GainLife {
        player: ctx.controller,
        amount: 2,
    });
    effects
}

fn iif_gained_life(state: &GameState, _source: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::you_gained_life_this_turn(state, you)
}

fn end_step_blood_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let blood = reg.interner().lookup("Blood").expect("Blood interned during register()");
    let mut blood_subtypes = SubtypeSet::default();
    blood_subtypes.0.insert(blood);
    let token = TokenDefinition {
        name: blood,
        colors: ColorSet::colorless(),
        types: TypeLine::ARTIFACT.into(),
        subtypes: blood_subtypes,
        power: None,
        toughness: None,
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}
