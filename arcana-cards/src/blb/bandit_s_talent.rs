//! Bandit's Talent — `{1}{B}` black Enchantment — Class.
//! Level 1 (ETB): Each opponent discards two cards unless they discard a nonland card.
//! Level 2: At the beginning of each opponent's upkeep, if that player has one or fewer
//!          cards in hand, they lose 2 life.
//! Level 3: At the beginning of your draw step, draw an additional card for each opponent
//!          who has one or fewer cards in hand.
//!
//! GAP: Level 1 ETB — "discard two cards unless they discard a nonland card" is a
//! conditional-discard gate; OptionalPaymentKind does not support Discard as a cost.
//! Approximating with Discard { count: 2 } for each opponent (the "unless nonland" choice
//! is dropped).
//! GAP: Level 2 trigger — "that player" refers to the currently-active opponent (the one
//! whose upkeep it is); the script API does not expose the active player directly.
//! Approximating by checking all opponents simultaneously.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bandit's Talent");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // Level 1 ETB: each opponent discards two cards (unless nonland — GAP)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2: opponent upkeep life loss if hand ≤ 1
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: upkeep_life_loss,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 3: draw step draw for each low-hand opponent
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Draw,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: draw_step_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2 activation
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}: Level 2".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up,
            })
            // Level 3 activation
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}: Level 3".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up,
            }),
    )
}

fn etb_discard(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "unless they discard a nonland card" conditional discard not expressible
    // Applying discard 2 to each opponent as best effort.
    script::opponents(state, trig.controller)
        .into_iter()
        .flat_map(|p| {
            vec![
                Effect::Discard { player: p, count: 2, choice: DiscardChoice::ControllerChooses },
            ]
        })
        .collect()
}

fn upkeep_life_loss(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: fires for all opponents simultaneously instead of just the active one
    script::opponents(state, trig.controller)
        .into_iter()
        .filter_map(|p| {
            if script::hand_size(state, p) <= 1 {
                Some(Effect::LoseLife { player: p, amount: 2 })
            } else {
                None
            }
        })
        .collect()
}

fn draw_step_draw(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let count = script::opponents(state, trig.controller)
        .into_iter()
        .filter(|&p| script::hand_size(state, p) <= 1)
        .count() as u32;
    if count == 0 {
        Vec::new()
    } else {
        vec![Effect::DrawCards { player: trig.controller, count }]
    }
}

fn level_up(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}
