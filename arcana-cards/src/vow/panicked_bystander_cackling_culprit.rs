//! Panicked Bystander // Cackling Culprit — `{1}{W}` Human Peasant 2/2 (front) /
//! Human Rogue (back). Transform card.
//!
//! Front face:
//!   Whenever this creature or another creature you control dies, you gain 1 life.
//!   At the beginning of your end step, if you gained 3 or more life this turn,
//!   transform this creature.
//!
//! Back face:
//!   Whenever this creature or another creature you control dies, you gain 1 life.
//!   {1}{B}: This creature gains deathtouch until end of turn.
//!
//! The "creature you control dies → gain 1 life" trigger is on the shared
//! CardDefinition and fires on BOTH faces (both Panicked Bystander and Cackling
//! Culprit print it), so it is left ungated. The end-step transform trigger is
//! front-face-only (gated to face 0) and now carries its intervening-if ("if you
//! gained 3 or more life this turn") via `script::life_gained_this_turn`. The
//! back-face activated ability ({1}{B}: this creature gains deathtouch until end
//! of turn) is wired and gated to the back face (face_gate: Some(1)).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Panicked Bystander");
    let human_sub = reg.interner_mut().intern("Human");
    let peasant_sub = reg.interner_mut().intern("Peasant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(peasant_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // Back face: Cackling Culprit — Human Rogue 2/2
    let back_name = reg.interner_mut().intern("Cackling Culprit");
    let back_human = reg.interner_mut().intern("Human");
    let back_rogue = reg.interner_mut().intern("Rogue");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_human);
    back_subtypes.0.insert(back_rogue);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Trigger 1: whenever a creature you control dies, gain 1 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: on_creature_dies,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Trigger 2: at the beginning of your end step, if you gained 3 or
            // more life this turn, transform. Front-face only.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(iif_gained_three_life),
                effect: on_end_step_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Back face: "{1}{B}: This creature gains deathtouch until end of turn."
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}: This creature gains deathtouch until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: Some(1), // back face only
                effect: gain_deathtouch,
            })
            // The end-step transform fires only on the front face. (Trigger 1,
            // the dies→gain-life, is intentionally ungated: it prints on both faces.)
            .with_trigger_face_gate(2, 0),
    )
}

fn iif_gained_three_life(
    state: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    arcana_core::script::life_gained_this_turn(state, you) >= 3
}

fn gain_deathtouch(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GrantKeyword {
        target: ctx.source,
        keyword: KeywordAbility::Deathtouch,
        duration: Duration::EndOfTurn,
    }]
}

fn on_creature_dies(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainLife {
        player: trig.controller,
        amount: 1,
    }]
}

fn on_end_step_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Gated by the intervening-if "if you gained 3+ life this turn".
    vec![Effect::Transform { target: trig.source }]
}
