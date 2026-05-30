//! Elusive Tormentor // Insidious Mist — `{2}{B}{B}` Vampire Wizard creature 4/4 (front) /
//! Elemental (back). Transform TDFC.
//!
//! Front face:
//!   {1}, Discard a card: Transform this creature.
//!
//! Back face (Insidious Mist):
//!   Hexproof, indestructible.
//!   This creature can't block and can't be blocked.
//!   Whenever this creature attacks and isn't blocked, you may pay {2}{B}. If you do, transform it.
//!
//! # GAPs
//! - Front activated ability: `{1}, Discard a card` — the {1} mana part is modeled via
//!   `ActivationCost.mana_cost`; the discard-a-card part maps to `discard_other`.
//! - Back face "can't block" static ability: not modeled (no static-ability API in engine).
//! - Back face trigger ("attacks and isn't blocked") uses `SelfAttacksUnblocked` as the
//!   closest available TriggerCondition.
//! - GAP: face-gating for triggered abilities not supported — the back-face unblocked trigger
//!   fires on both faces.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elusive Tormentor");
    let vampire_sub = reg.interner_mut().intern("Vampire");
    let wizard_sub = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire_sub);
    subtypes.0.insert(wizard_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // Back face: Insidious Mist — Elemental, hexproof, indestructible
    // GAP: "can't block" static ability not modeled
    // GAP: P/T on back is 0/1 per oracle; using those values
    let back_name = reg.interner_mut().intern("Insidious Mist");
    let elemental_sub = reg.interner_mut().intern("Elemental");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(elemental_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Hexproof, KeywordAbility::Indestructible],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: {1}, Discard a card: Transform
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, Discard a card: Transform this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    discard_other: Some(ObjectFilter::default()),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(0), // front face only
                effect: front_transform,
            })
            // Back face: whenever attacks and isn't blocked, may pay {2}{B} to transform back
            // GAP: face-gating triggered abilities not supported — fires on both faces
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacksUnblocked,
                intervening_if: None,
                effect: back_unblocked_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn front_transform(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn back_unblocked_trigger(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Mana(ManaCost::parse("{2}{B}").expect("valid cost")),
        then: Box::new(Effect::Transform { target: trig.source }),
        else_effect: None,
    }]
}
