//! Hanweir Militia Captain // Westvale Cult Leader — `{1}{W}` Creature — Human Soldier 2/2 (front) /
//! Creature — Human Cleric (back). Transform.
//!
//! Front: At the beginning of your upkeep, if you control four or more creatures, transform.
//!
//! Back: Westvale Cult Leader's power and toughness are each equal to the number of creatures you control.
//!   At the beginning of your end step, create a 1/1 white and black Human Cleric creature token.
//!
//! "if you control four or more creatures" intervening-if modeled via
//!   `conditions::you_control_at_least` on the front-face upkeep transform trigger.
//! GAP: Back face dynamic P/T (equal to number of creatures you control) not expressible as a
//!   characteristic (PtValue has only Fixed/Star/StarPlus, no board-count CDA); back face is
//!   registered as a fixed-size creature with placeholder 0/0 stats.

use arcana_core::conditions;
use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::objects::ObjectId;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hanweir Militia Captain");

    let human_sub = reg.interner_mut().intern("Human");
    let soldier_sub = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(soldier_sub);

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

    let back_name = reg.interner_mut().intern("Westvale Cult Leader");
    let back_human_sub = reg.interner_mut().intern("Human");
    let cleric_sub = reg.interner_mut().intern("Cleric");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(back_human_sub);
    back_subtypes.0.insert(cleric_sub);

    // Pre-intern token subtypes for back-face end-step token creation
    let token_human_sub = reg.interner_mut().intern("Human");
    let token_cleric_sub = reg.interner_mut().intern("Cleric");

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            // GAP: dynamic P/T; placeholder 0/0 — actual value equals creatures you control.
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            ..Default::default()
        },
        spell_ability: None,
    };

    let _ = (token_human_sub, token_cleric_sub); // interned for lookup at resolve time

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: at beginning of your upkeep, if 4+ creatures, transform.
            // Intervening-if "if you control four or more creatures" via conditions::you_control_at_least.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(iif_four_creatures),
                effect: front_upkeep_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face: at beginning of your end step, create a 1/1 white and black Human Cleric token.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: back_end_step_token,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(1, 0) // front upkeep transform — front face only
            .with_trigger_face_gate(2, 1), // back end-step token — back face only
    )
}

fn iif_four_creatures(state: &GameState, _source: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    conditions::you_control_at_least(state, you, &ObjectFilter::new().with_types(TypeLine::CREATURE.into()), 4)
}

fn front_upkeep_transform(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // The "4+ creatures" gate is enforced by the trigger's intervening_if; re-checked
    // here so the resolution is a no-op if the board shrank between trigger and resolution.
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let count = script::count_matching(state, &filter, trig.controller);
    if count >= 4 {
        vec![Effect::Transform { target: trig.source }]
    } else {
        Vec::new()
    }
}

fn back_end_step_token(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let human_name = reg.interner().lookup("Human")
        .expect("Human interned during register()");
    let cleric_id = reg.interner().lookup("Cleric")
        .expect("Cleric interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(human_name);
    token_subtypes.0.insert(cleric_id);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: human_name,
            colors: ColorSet::white() | ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}
