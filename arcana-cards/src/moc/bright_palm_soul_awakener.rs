//! Bright-Palm, Soul Awakener — `{1}{R}{G}{W}` 4/3 Legendary Fox Shaman.
//!
//! Oracle:
//! * Backup 1 (When this creature enters, put a +1/+1 counter on target
//!   creature. If that's another creature, it gains the following ability
//!   until end of turn.) — keyword not in usable surface; ETB counter modeled
//!   directly, the conditional ability-grant rider GAP'd.
//! * Whenever this creature attacks, double the number of +1/+1 counters on
//!   target creature. That creature can't be blocked by creatures with power 2
//!   or less this turn.
//!
//! Backup is not a usable keyword variant; its ETB ("put a +1/+1 counter on
//! target creature") is emitted as a triggered ability. The Backup rider ("if
//! that's another creature, it gains the following ability") is a conditional
//! ability grant with no expressible primitive and is GAP'd. The attacks
//! trigger doubles the +1/+1 counters on a target creature (adds a number
//! equal to the current count); the "can't be blocked by creatures with power
//! 2 or less" clause is a power-filtered can't-be-blocked with no expressible
//! filtered primitive (CantBeBlocked is unfiltered) and is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bright-Palm, Soul Awakener");
    let fox = reg.interner_mut().intern("Fox");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fox);
    subtypes.0.insert(shaman);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: keyword — Backup 1 is not a usable KeywordAbility variant;
        // its ETB effect is modeled as the triggered ability below.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: backup_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: double_counters_on_target,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn backup_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: Backup rider — "If that's another creature, it gains the following
    // ability until end of turn." Conditional ability grant not expressible.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn double_counters_on_target(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let current = state
        .objects
        .get(*id)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    // GAP: "That creature can't be blocked by creatures with power 2 or less
    // this turn." Power-filtered can't-be-blocked is not expressible
    // (CantBeBlocked is unfiltered).
    if current == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: current,
    }]
}
