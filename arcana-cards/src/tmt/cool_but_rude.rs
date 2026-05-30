//! Cool but Rude — `{1}{R}` Enchantment — Class (red)
//!
//! Level 1: Whenever you attack, you may discard a card. If you do, draw a card.
//! Level 2 ({1}{R}): Whenever you discard a card, this Class deals 2 damage to each opponent.
//! Level 3 ({1}{R}): When this Class becomes level 3, search your library for a card,
//!   put it into your hand, shuffle, then discard a card at random.
//!
//! GAP: Level 1 "you may discard a card, if you do draw a card" — OptionalPaymentKind
//!   does not support Discard as a cost; level-1 trigger fires but returns Vec::new().
//! GAP: Level 2 "whenever you discard a card, deals 2 damage to each opponent" —
//!   per-level trigger gating not implemented; trigger fires regardless of level.
//! GAP: Level 3 "when this becomes level 3" — modeled as effect in level-up-to-3 activation.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cool but Rude");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
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
            // Level 1: Whenever you attack, you may discard a card. If you do, draw a card.
            // GAP: discard-to-draw loot not expressible (OptionalPaymentKind has no Discard).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: on_attack_loot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2: Whenever you discard a card, deal 2 damage to each opponent.
            // GAP: per-level ability gating not implemented — fires regardless of level.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CardDiscarded {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: on_discard_damage_opponents,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").unwrap(),
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
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").unwrap(),
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
            }),
    )
}

fn on_attack_loot(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may discard a card, if you do draw a card" —
    // OptionalPaymentKind does not support Discard as a cost.
    Vec::new()
}

fn on_discard_damage_opponents(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Deals 2 damage to each opponent.
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::DealDamage {
            target: DamageTarget::Player(opp),
            amount: 2,
            source: trig.source,
        })
        .collect()
}

fn level_up_to_2(
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

fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // When this Class becomes level 3, search for a card, put into hand, shuffle,
    // then discard a card at random.
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Level,
            count: 1,
        },
        Effect::TutorToHand {
            player: ctx.controller,
            filter: ObjectFilter::new(),
            reveal: false,
        },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::Random,
        },
    ]
}
