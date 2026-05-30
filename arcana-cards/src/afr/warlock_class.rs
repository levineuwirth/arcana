//! Warlock Class — `{B}` black Enchantment — Class.
//! Level 1: At the beginning of your end step, if a creature died this turn,
//!   each opponent loses 1 life.
//! Level 2 ({1}{B}): When this Class becomes level 2, look at the top three
//!   cards of your library. Put one of them into your hand and the rest into
//!   your graveyard.
//! Level 3 ({6}{B}): At the beginning of your end step, each opponent loses
//!   life equal to the life they lost this turn.
//!
//! GAP: Level 1 "if a creature died this turn" — there is no script helper
//!   for "any creature died this turn" (only creatures_of_subtype_died_this_turn).
//!   The conditional is NOT checked; the effect fires unconditionally.
//! GAP: Level 3 "each opponent loses life equal to the life they lost this turn"
//!   — there is no script helper for opponent life-lost-this-turn. Effect
//!   is omitted (Vec::new()) for the level-3 end-step trigger.
//! GAP: Level-gating triggers (level 1/3 end-step abilities should only fire
//!   at the correct level) — the engine does not support level-gated triggers.

use arcana_core::effects::{DigRest, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Warlock Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 1: at beginning of end step, (if creature died) each opponent loses 1 life
            // GAP: conditional "if a creature died this turn" not checked (no script helper).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_level1,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2 activation
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_2,
            })
            // Level 3 activation
            .with_activated_ability(ActivatedAbilityDef {
                text: "{6}{B}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{6}{B}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_3,
            })
            // Level 3: at beginning of end step, each opponent loses life equal to life they lost
            // GAP: "life they lost this turn" not computable — omitted.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_level3,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Level 1 end-step: each opponent loses 1 life.
/// GAP: fires unconditionally (should only fire if a creature died this turn).
fn end_step_level1(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    arcana_core::script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::LoseLife { player: opp, amount: 1 })
        .collect()
}

/// Level 2 activation: add a Level counter and look at top 3 cards.
fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Level,
            count: 1,
        },
        Effect::DigTopN {
            player: ctx.controller,
            count: 3,
            filter: None,
            rest: DigRest::Graveyard,
        },
    ]
}

/// Level 3 activation: add a Level counter.
fn level_up_to_3(
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

/// Level 3 end-step trigger: each opponent loses life equal to life they lost this turn.
/// GAP: no script helper for opponent life-lost-this-turn; emitting Vec::new().
fn end_step_level3(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "each opponent loses life equal to the life they lost this turn" —
    // no script helper for life-lost-this-turn by a player.
    Vec::new()
}
