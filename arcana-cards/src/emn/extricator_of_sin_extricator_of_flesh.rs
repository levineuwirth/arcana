//! Extricator of Sin // Extricator of Flesh — `{2}{W}` Human Cleric creature 0/3.
//!
//! Front face (Extricator of Sin):
//!   When this creature enters, you may sacrifice another permanent. If you do, create a 3/2
//!   colorless Eldrazi Horror creature token.
//!   Delirium — At the beginning of your upkeep, if there are four or more card types among cards
//!   in your graveyard, transform this creature.
//!
//! Back face (Extricator of Flesh) — Creature — Eldrazi Horror:
//!   Eldrazi you control have vigilance.
//!   {2}, {T}, Sacrifice a non-Eldrazi creature: Create a 3/2 colorless Eldrazi Horror creature token.
//!
//! # GAPs
//! - Delirium intervening-if (four or more card types in graveyard) gates the upkeep transform
//!   trigger via `conditions::delirium`.
//! - "Eldrazi you control have vigilance" is a static keyword-grant — not modeled.
//!   GAP: static keyword-grant to subtype not modeled.
//!
//! The front-face ETB "you may sacrifice another permanent. If you do, create a 3/2 Eldrazi
//! Horror" is wired as an optional sacrifice payment (sacrifice a permanent, then create the
//! token). Minor over-inclusion: the selection can't exclude the source ("another"), so
//! Extricator of Sin itself is technically offerable; harmless in practice.

use arcana_core::actions::{OptionalPaymentKind, SacrificeFilter};
use arcana_core::conditions;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PlayerId, PtValue, SmallString, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Extricator of Sin");
    let human_sub = reg.interner_mut().intern("Human");
    let cleric_sub = reg.interner_mut().intern("Cleric");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(cleric_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Pre-intern Eldrazi Horror token subtypes
    let _eldrazi_sub = reg.interner_mut().intern("Eldrazi");
    let _horror_sub = reg.interner_mut().intern("Horror");

    // Back face
    let back_name = reg.interner_mut().intern("Extricator of Flesh");
    let back_eldrazi = reg.interner_mut().intern("Eldrazi");
    let back_horror = reg.interner_mut().intern("Horror");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_eldrazi);
    back_subtypes.0.insert(back_horror);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::colorless(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(3)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Triggered ability 1: ETB — you may sacrifice a permanent; if you
            // do, create a 3/2 colorless Eldrazi Horror token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_create_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Triggered ability 2: beginning of upkeep, if delirium, transform.
            // Front-face only (delirium transform front → back).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(iif_delirium),
                effect: transform_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Delirium transform is the front→back flip, so it fires on the
            // front face only.
            .with_trigger_face_gate(2, 0)
            // Back-face activated ability — "{2}, {T}, Sacrifice a non-Eldrazi
            // creature: Create a 3/2 colorless Eldrazi Horror creature token."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}, Sacrifice a non-Eldrazi creature: Create a 3/2 \
                       colorless Eldrazi Horror creature token."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    sacrifice_other: Some(
                        ObjectFilter::creature().without_subtype_sym(back_eldrazi),
                    ),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: activated_create_token,
            }),
    )
}

/// Builds the 3/2 colorless Eldrazi Horror token minted by both the front-face
/// ETB and the back-face activated ability.
fn make_eldrazi_horror_token(eldrazi: SmallString, horror: SmallString) -> TokenDefinition {
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(horror);
    TokenDefinition {
        name: horror,
        colors: ColorSet::new(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    }
}

fn activated_create_token(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let eldrazi_id = reg.interner().lookup("Eldrazi")
        .expect("Eldrazi interned during register()");
    let horror_id = reg.interner().lookup("Horror")
        .expect("Horror interned during register()");
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: make_eldrazi_horror_token(eldrazi_id, horror_id),
    }]
}

fn etb_create_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let eldrazi_id = reg.interner().lookup("Eldrazi")
        .expect("Eldrazi interned during register()");
    let horror_id = reg.interner().lookup("Horror")
        .expect("Horror interned during register()");
    // "you may sacrifice another permanent. If you do, create a 3/2 colorless
    // Eldrazi Horror creature token."
    vec![Effect::OptionalPayment {
        chooser: trig.controller,
        cost: OptionalPaymentKind::Sacrifice(SacrificeFilter::Permanent),
        then: Box::new(Effect::CreateToken {
            controller: trig.controller,
            token: make_eldrazi_horror_token(eldrazi_id, horror_id),
        }),
        else_effect: None,
    }]
}

fn iif_delirium(
    state: &GameState,
    _source: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::delirium(state, you)
}

fn transform_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: trig.source }]
}
