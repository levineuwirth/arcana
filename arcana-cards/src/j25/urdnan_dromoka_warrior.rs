//! Urdnan, Dromoka Warrior — `{1}{W}` 1/1 Legendary Human Warrior.
//!
//! * "When Urdnan enters, put a +1/+1 counter on target creature." — an
//!   ETB targeted-counter trigger.
//! * "Whenever you attack, target attacking creature with a +1/+1 counter
//!   on it gains first strike until end of turn. If that creature has two
//!   or more +1/+1 counters on it, it gains double strike instead." —
//!   modeled as `CreatureAttacks` (you control) targeting an attacking
//!   creature that already has a +1/+1 counter; the resolver branches on
//!   the live counter count to grant double strike vs. first strike.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urdnan, Dromoka Warrior");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };

    // "target attacking creature with a +1/+1 counter on it"
    let mut attacking_with_counter = ObjectFilter::creature();
    attacking_with_counter.combat_status =
        Some(arcana_core::targets::CombatStatusFilter::Attacking);
    attacking_with_counter.has_counter = Some(CounterKind::PlusOnePlusOne);

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                },
                intervening_if: None,
                effect: grant_strike,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(attacking_with_counter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn etb_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn grant_strike(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let counters = state
        .objects
        .get(*id)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    let keyword = if counters >= 2 {
        KeywordAbility::DoubleStrike
    } else {
        KeywordAbility::FirstStrike
    };
    vec![Effect::GrantKeyword {
        target: *id,
        keyword,
        duration: Duration::EndOfTurn,
    }]
}
