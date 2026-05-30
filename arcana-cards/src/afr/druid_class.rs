//! Druid Class — {1}{G} Enchantment — Class
//!
//! Level 1: Landfall — Whenever a land you control enters, you gain 1 life.
//! Level 2 ({2}{G}): You may play an additional land on each of your turns.
//!   GAP: "play an additional land" is a replacement/continuous effect, not a P/T or keyword anthem;
//!   deferred (continuous-effect engine subsystem).
//! Level 3 ({4}{G}): When this Class becomes level 3, target land you control becomes a creature
//!   with haste and "This creature's power and toughness are each equal to the number of lands you
//!   control." It's still a land.
//!   GAP: per-level granted ability deferred (continuous-effect engine subsystem) — dynamic P/T
//!   equal to lands controlled and land-becomes-creature cannot be expressed with current API.
//! Keyword note: Landfall is not a keyword in the engine keyword list; the Landfall trigger is
//!   modeled as a ZoneChange triggered ability.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationZone, CardDefinition, CardRegistry,
    EntersWithSpec,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::state::GameState;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Druid Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let land_filter = ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 1 Landfall: whenever a land you control enters, gain 1 life.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: land_filter,
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: landfall_gain_life,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Level 2 activation
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").expect("valid cost"),
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
                text: "{4}{G}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{G}").expect("valid cost"),
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

fn landfall_gain_life(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 1 }]
}

fn level_up_to_2(_state: &GameState, ctx: &arcana_core::registry::ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: per-level granted ability deferred (continuous-effect engine subsystem)
    // "You may play an additional land on each of your turns" requires a continuous replacement
    // effect that is not yet expressible.
    vec![
        Effect::AddCounters { target: ctx.source, kind: CounterKind::Level, count: 1 },
    ]
}

fn level_up_to_3(_state: &GameState, ctx: &arcana_core::registry::ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: per-level granted ability deferred (continuous-effect engine subsystem)
    // "Target land you control becomes a creature with haste and dynamic P/T equal to lands
    // controlled" — land-becomes-creature with dynamic P/T is not expressible with current API.
    vec![
        Effect::AddCounters { target: ctx.source, kind: CounterKind::Level, count: 1 },
    ]
}
