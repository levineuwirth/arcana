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
//! - ETB "you may sacrifice another permanent. If you do," gate — OptionalPaymentKind does not
//!   support Sacrifice; the token is created unconditionally as best-effort.
//!   GAP: sacrifice-gate on ETB not expressible via OptionalPaymentKind.
//! - Delirium intervening-if (four or more card types in graveyard) gates the upkeep transform
//!   trigger via `conditions::delirium`.
//! - "Eldrazi you control have vigilance" is a static keyword-grant — not modeled.
//!   GAP: static keyword-grant to subtype not modeled.
//! - Back face activated ability ({2},{T}, Sacrifice non-Eldrazi: create token) not modeled
//!   (sacrifice-other activation cost is not supported).
//!   GAP: back-face activated ability (sacrifice-other cost) not modeled.
//! - Back face triggered abilities are not auto-installed on transform.
//!   GAP: back-face-only triggered ability not modeled.

use arcana_core::conditions;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
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
            // Triggered ability 1: ETB — create a 3/2 colorless Eldrazi Horror token
            // GAP: the "you may sacrifice another permanent. If you do," gate is not expressible;
            // token is created unconditionally.
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
            }),
    )
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
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi_id);
    subtypes.0.insert(horror_id);
    let token = TokenDefinition {
        name: horror_id,
        colors: ColorSet::new(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
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
