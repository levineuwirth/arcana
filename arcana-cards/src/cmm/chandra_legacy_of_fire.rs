//! Chandra, Legacy of Fire — `{4}{R}` legendary planeswalker, starting loyalty 4.
//!
//! End-step trigger: At the beginning of your end step, Chandra deals X
//!   damage to each opponent, where X is the number of planeswalkers you
//!   control.
//! +1: Add {R} for each planeswalker you control.
//! 0: Remove a loyalty counter from each of any number of permanents you
//!    control; exile that many cards from the top of your library; you
//!    may play them this turn (dynamic-X cost-pay + play-from-exile GAP).
//!
//! Scope: the end-step ping (X = planeswalkers you control, to each
//! opponent) and the +1 (add {R} per planeswalker) are fully expressed.
//! The 0 ability — remove-any-number-of-loyalty-counters and exile-that-
//! many with a play-this-turn rider — has no demonstrated Effect.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chandra, Legacy of Fire");
    let chandra = reg.interner_mut().intern("Chandra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(chandra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_ping,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Add {R} for each planeswalker you control.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Remove a loyalty counter from each of any number of \
                       permanents you control. Exile that many cards from the \
                       top of your library. You may play them this turn.".into(),
                cost: ActivationCost::default(),
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_impulse,
            }),
    )
}

fn planeswalker_filter() -> ObjectFilter {
    ObjectFilter::permanent()
        .with_types(TypeLine::PLANESWALKER.into())
        .controlled_by(ControllerConstraint::You)
}

/// End step — deal X to each opponent (X = planeswalkers you control).
fn end_step_ping(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::count_matching(state, &planeswalker_filter(), trig.controller);
    let effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|p| Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: x,
        })
        .collect();
    vec![Effect::Sequence(effects)]
}

/// `+1: Add {R} for each planeswalker you control.`
fn plus_one_mana(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::count_matching(state, &planeswalker_filter(), ctx.controller);
    let mana: Vec<ManaUnit> = (0..n)
        .map(|_| ManaUnit::plain(ManaColor::Red, ctx.source))
        .collect();
    vec![Effect::AddMana { player: ctx.controller, mana }]
}

/// `0` — remove-any-number-of-loyalty + impulse exile with play rider.
fn zero_impulse(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "remove a loyalty counter from each of any number of permanents
    // you control, exile that many, you may play them this turn" — a
    // dynamic cost-payment + play-from-exile rider not expressible.
    Vec::new()
}
