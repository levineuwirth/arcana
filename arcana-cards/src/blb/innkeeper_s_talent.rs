//! Innkeeper's Talent — `{1}{G}` green Enchantment — Class.
//! Level 1: At the beginning of combat on your turn, put a +1/+1 counter on target creature you control.
//! Level 2 ({G}): Permanents you control with counters on them have ward {1}.
//! Level 3 ({3}{G}): If you would put one or more counters on a permanent or player, put twice that many instead.
//! GAP: per-level granted abilities deferred (continuous-effect engine subsystem).
//! GAP: Level 2/3 continuous effects — not in catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Innkeeper's Talent");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics { name, mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")), colors: ColorSet::green(), types: TypeLine::ENCHANTMENT.into(), subtypes, supertypes: SupertypeSet::default(), ..Default::default() };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters { kind: CounterKind::Level, count: 1 })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1, trigger_condition: TriggerCondition::PhaseBegins { phase: Phase::Combat, whose: ControllerConstraint::You },
                intervening_if: None, effect: combat_counter, trigger_zones: vec![Zone::Battlefield], frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement { filter: TargetFilter::Permanent(ObjectFilter::creature().controlled_by(ControllerConstraint::You)), count: TargetCount::Exactly(1), controller: None }],
            })
            .with_activated_ability(ActivatedAbilityDef { text: "{G}: Level 2.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{G}").unwrap(), min_self_counters: Some((CounterKind::Level, 1)), ..ActivationCost::default() }, target_requirements: vec![], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: false, face_gate: None, effect: level_up_to_2 })
            .with_activated_ability(ActivatedAbilityDef { text: "{3}{G}: Level 3.".into(), cost: ActivationCost { mana_cost: ManaCost::parse("{3}{G}").unwrap(), min_self_counters: Some((CounterKind::Level, 2)), ..ActivationCost::default() }, target_requirements: vec![], is_mana_ability: false, is_loyalty_ability: false, activation_zone: ActivationZone::Battlefield, is_instant_speed: false, face_gate: None, effect: level_up_to_3 }),
    )
}

fn combat_counter(_state: &GameState, trig: &PendingTrigger, _: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 1 }]
}

fn level_up_to_2(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters { target: ctx.source, kind: CounterKind::Level, count: 1 }]
}

fn level_up_to_3(_state: &GameState, ctx: &ActivationContext, _: &CardRegistry) -> Vec<Effect> {
    // GAP: per-level granted abilities deferred (continuous-effect engine subsystem)
    vec![Effect::AddCounters { target: ctx.source, kind: CounterKind::Level, count: 1 }]
}
