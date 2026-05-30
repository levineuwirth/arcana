//! Gossip's Talent — `{1}{U}` Enchantment — Class (blue).
//! Level 1: Whenever a creature you control enters, surveil 1.
//! Level 2 ({1}{U}): Whenever you attack, target attacking creature with power 3 or less
//!   can't be blocked this turn.
//! Level 3 ({3}{U}): Whenever a creature you control deals combat damage to a player, you
//!   may exile it, then return it to the battlefield under its owner's control.
//!
//! GAP: Level 2 triggered ability "whenever you attack, target attacking creature with power
//!   3 or less can't be blocked this turn" — per-level triggered ability installation is not
//!   modeled (install-on-level-up only supports P/T and keyword anthems). Level-up counter
//!   is correctly incremented.
//! GAP: Level 3 triggered ability "whenever a creature you control deals combat damage to a
//!   player, you may exile it, then return it to the battlefield" — per-level triggered ability
//!   installation not modeled. Level-up counter correctly incremented.
//! NOTE: Scryfall keyword "Surveil" refers to Level 1's triggered ability.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gossip's Talent");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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
            // Level 1: whenever a creature you control enters, surveil 1.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: surveil_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2: {1}{U} — whenever you attack, target attacking creature with
            // power 3 or less can't be blocked this turn.
            // GAP: per-level triggered ability installation not modeled; only the
            // Level counter increment fires.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}: Level 2".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").unwrap(),
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
            // Level 3: {3}{U} — whenever a creature you control deals combat damage
            // to a player, you may exile it, then return it to the battlefield.
            // GAP: per-level triggered ability installation not modeled.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: Level 3".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").unwrap(),
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

fn surveil_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Surveil { player: trig.controller, count: 1 }]
}

/// Level 2: increment the Level counter.
/// GAP: "whenever you attack, target attacking creature with power 3 or less can't be
/// blocked this turn" — per-level triggered ability installation not modeled.
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

/// Level 3: increment the Level counter.
/// GAP: "whenever a creature you control deals combat damage to a player, you may exile it,
/// then return it to the battlefield under its owner's control" — per-level triggered ability
/// installation not modeled (continuous-effect engine subsystem).
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
