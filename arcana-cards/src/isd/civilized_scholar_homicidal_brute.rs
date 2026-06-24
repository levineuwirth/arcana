//! Civilized Scholar // Homicidal Brute — `{2}{U}` blue Human Advisor 0/1 (front) /
//! Human Mutant (back). Transform creature.
//!
//! Front face (Civilized Scholar):
//!   {T}: Draw a card, then discard a card. If a creature card is discarded this way,
//!        untap this creature, then transform it.
//!
//! Back face (Homicidal Brute):
//!   At the beginning of your end step, if this creature didn't attack this turn,
//!   tap this creature, then transform it.
//!
//! GAP: Front-face activation "if a creature card is discarded this way, untap then transform"
//!   — checking whether the specifically discarded card was a creature card is not expressible
//!   in the Effect catalog (no conditional-on-discarded-card-type variant). Modeled as:
//!   draw + discard, then unconditionally untap + transform. (The transform fires always,
//!   not only when a creature card is discarded — verify will flag this.)
//!
//! Back-face trigger ("At the beginning of your end step, if this creature didn't attack
//! this turn, tap this creature, then transform it") IS now wired, face-gated to the back
//! face: StepBegins(End, You) + intervening-if !source_attacked_this_turn + Tap + Transform.
use arcana_core::conditions;
use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Civilized Scholar");
    let human_sub = reg.interner_mut().intern("Human");
    let advisor_sub = reg.interner_mut().intern("Advisor");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(advisor_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // Back face: Homicidal Brute
    let back_name = reg.interner_mut().intern("Homicidal Brute");
    let human_sub2 = reg.interner_mut().intern("Human");
    let mutant_sub = reg.interner_mut().intern("Mutant");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub2);
    back_subtypes.0.insert(mutant_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::blue(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(1)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {T}: draw a card, discard a card, then (unconditionally in this
            // approximation) untap and transform.
            // GAP: should only untap+transform if discarded card was a creature card.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Draw a card, then discard a card. If a creature card is discarded this way, untap this creature, then transform it.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0),
                effect: scholar_tap,
            })
            // Back face (Homicidal Brute): "At the beginning of your end step, if this
            // creature didn't attack this turn, tap this creature, then transform it."
            // Face-gated to the back face (1); the intervening-if negates
            // `source_attacked_this_turn`.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_did_not_attack),
                effect: brute_tap_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 1),
    )
}

fn if_did_not_attack(
    s: &GameState,
    src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    !conditions::source_attacked_this_turn(s, src)
}

fn brute_tap_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Tap { target: trig.source },
        Effect::Transform { target: trig.source },
    ]
}

fn scholar_tap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: unconditional transform; should only fire when a creature card is discarded.
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::Discard { player: ctx.controller, count: 1, choice: DiscardChoice::ControllerChooses },
        Effect::Untap { target: ctx.source },
        Effect::Transform { target: ctx.source },
    ]
}
