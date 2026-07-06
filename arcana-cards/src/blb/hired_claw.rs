//! Hired Claw — `{R}` 1/2 Lizard Mercenary.
//!
//! Oracle:
//! * Whenever you attack with one or more Lizards, this creature deals 1
//!   damage to target opponent.
//! * {1}{R}: Put a +1/+1 counter on this creature. Activate only if an
//!   opponent lost life this turn and only once each turn.
//!
//! Decomposition:
//! 1. No keyword line.
//! 2. Attack trigger → 1 damage to target opponent. Modeled with
//!    CreatureAttacks filtered to Lizards you control. NOTE: the printed
//!    trigger is a once-per-combat batch ("one or more Lizards"); the engine
//!    fires CreatureAttacks per attacking creature, so this over-fires when
//!    several Lizards attack — a fidelity gap, the closest expressible form.
//! 3. {1}{R} activated → +1/+1 counter on self; gated by once_per_turn and an
//!    activation_condition "an opponent lost life this turn".

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::conditions;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hired Claw");
    let lizard = reg.interner_mut().intern("Lizard");
    let mercenary = reg.interner_mut().intern("Mercenary");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(mercenary);

    // Lizard filter (you control) for the attack trigger.
    let lizard_filter =
        script::subtype_filter(reg, "Lizard").controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: lizard_filter,
                },
                intervening_if: None,
                effect: damage_opponent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_opponent()],
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: Put a +1/+1 counter on this creature. Activate only if an opponent lost life this turn and only once each turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    once_per_turn: true,
                    activation_condition: Some(if_opponent_lost_life),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_counter,
            }),
    )
}

fn damage_opponent(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: DamageTarget::Player(*p),
        amount: 1,
    }]
}

fn if_opponent_lost_life(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::an_opponent_lost_life_this_turn(s, you)
}

fn add_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
