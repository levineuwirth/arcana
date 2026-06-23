//! Conclave Sledge-Captain — `{5}{G}` 4/4 Elephant Soldier.
//!
//! * Backup 1, backup 1, backup 1 — three separate ETB triggers, each:
//!   "put a +1/+1 counter on target creature. If that's another creature,
//!   it gains [trample] until end of turn." The +1/+1 counter is faithfully
//!   modeled per backup trigger; the keyword-grant rider on a *different*
//!   target is GAP'd (Backup is not a modeled keyword and the conditional
//!   "if that's another creature" grant has no primitive here).
//! * Trample.
//! * Whenever this creature deals combat damage to a player, put that many
//!   +1/+1 counters on it.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::targets::TargetFilter;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Conclave Sledge-Captain");
    let elephant = reg.interner_mut().intern("Elephant");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elephant);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(backup_def(1))
            .with_triggered_ability(backup_def(2))
            .with_triggered_ability(backup_def(3))
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: arcana_core::targets::ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// One Backup-1 ETB instance: put a +1/+1 counter on target creature.
fn backup_def(id: u32) -> TriggeredAbilityDef {
    TriggeredAbilityDef {
        id,
        trigger_condition: TriggerCondition::SelfEntersBattlefield,
        intervening_if: None,
        effect: backup_counter,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: vec![TargetRequirement::target_creature()],
    }
}

fn backup_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: "if that's another creature, it gains [trample] until end of turn"
    // — the conditional keyword grant on the chosen target is not modeled.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

/// "put that many +1/+1 counters on it" where "that many" is the combat
/// damage just dealt to the player, and "it" is this creature.
fn combat_damage_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: n,
    }]
}
