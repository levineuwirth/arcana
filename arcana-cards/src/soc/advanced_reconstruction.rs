//! Advanced Reconstruction — {3}{R} Enchantment — Class
//!
//! Level 1: At the beginning of your first main phase, mill a card, then exile a card from your
//!   graveyard at random. You may play the exiled card this turn.
//!   GAP: "exile a card from your graveyard at random then may play it this turn" — the
//!   exile-from-graveyard-at-random + play-from-exile-this-turn chain is not expressible with
//!   current API. The Mill 1 portion is modeled; the rest is GAP.
//! Level 2 ({1}{R}): Whenever one or more cards leave your graveyard, this Class deals 2 damage
//!   to each opponent.
//!   GAP: TriggerCondition for "cards leave graveyard" is not available in current API.
//! Level 3 ({1}{R}): Spells you cast from anywhere other than your hand cost {2} less to cast.
//!   GAP: per-level granted ability deferred (continuous-effect engine subsystem) — cost
//!   reduction replacement effect not expressible.
//! Keyword note: Mill is a Scryfall keyword tag (not a KeywordAbility); no engine keyword emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationZone, CardDefinition, CardRegistry,
    EntersWithSpec,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::targets::ControllerConstraint;
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Advanced Reconstruction");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 1: At the beginning of your first main phase, mill 1.
            // GAP: The "exile a card from your graveyard at random; you may play it this turn"
            // portion is not expressible.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: level1_phase_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Level 2 activation
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
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
                text: "{1}{R}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
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
    )
}

fn level1_phase_trigger(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Mill 1. GAP: "exile a card from your graveyard at random; you may play it this turn"
    // is not expressible with current API.
    vec![
        Effect::Mill { player: trig.controller, count: 1 },
    ]
}

fn level_up_to_2(_state: &GameState, ctx: &arcana_core::registry::ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Whenever one or more cards leave your graveyard, this Class deals 2 damage to each
    // opponent" — triggered ability on level-up using a graveyard-leave condition is not
    // expressible with current API.
    vec![
        Effect::AddCounters { target: ctx.source, kind: CounterKind::Level, count: 1 },
    ]
}

fn level_up_to_3(_state: &GameState, ctx: &arcana_core::registry::ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: per-level granted ability deferred (continuous-effect engine subsystem)
    // "Spells you cast from anywhere other than your hand cost {2} less" — cost reduction
    // continuous effect not expressible.
    vec![
        Effect::AddCounters { target: ctx.source, kind: CounterKind::Level, count: 1 },
    ]
}
