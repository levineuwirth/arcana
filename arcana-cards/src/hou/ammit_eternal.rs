//! Ammit Eternal — `{2}{B}` 5/5 Creature — Zombie Crocodile Demon.
//!
//! * "Afflict 3 (Whenever this creature becomes blocked, defending player
//!   loses 3 life.)" — Afflict is not an expressible `KeywordAbility`, so it is
//!   modeled as its reminder-text triggered ability (SelfBecomesBlocked →
//!   defending player loses 3 life). The SelfBecomesBlocked event exposes no
//!   defending-player accessor, so the lone opponent (2-player) is used; this
//!   is a fidelity gap in multiplayer.
//! * Whenever an opponent casts a spell, put a -1/-1 counter on this creature.
//! * Whenever this creature deals combat damage to a player, remove all -1/-1
//!   counters from it.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ammit Eternal");
    let zombie = reg.interner_mut().intern("Zombie");
    let crocodile = reg.interner_mut().intern("Crocodile");
    let demon = reg.interner_mut().intern("Demon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(crocodile);
    subtypes.0.insert(demon);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: afflict_3,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SpellCast {
                    filter: None,
                    caster: ControllerConstraint::Opponent,
                },
                intervening_if: None,
                effect: add_minus_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: DamageDealt has no self-only source filter; restricted to a
            // creature you control. The effect only touches this creature's
            // counters, but the trigger over-fires on other creatures' combat
            // damage.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: source_self_filter(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: remove_all_minus_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn source_self_filter() -> ObjectFilter {
    ObjectFilter::creature().controlled_by(ControllerConstraint::You)
}

fn afflict_3(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Afflict: defending player loses 3 life. No defending-player accessor on
    // SelfBecomesBlocked — use the lone opponent (2-player fidelity gap).
    let opps = script::opponents(state, trig.controller);
    let Some(&p) = opps.first() else { return Vec::new(); };
    vec![Effect::LoseLife { player: p, amount: 3 }]
}

fn add_minus_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::MinusOneMinusOne,
        count: 1,
    }]
}

fn remove_all_minus_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = state
        .objects
        .get(trig.source)
        .map_or(0, |o| o.count_counters(CounterKind::MinusOneMinusOne));
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind: CounterKind::MinusOneMinusOne,
        count: n,
    }]
}
