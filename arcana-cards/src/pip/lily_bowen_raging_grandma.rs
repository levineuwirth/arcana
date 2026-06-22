//! Lily Bowen, Raging Grandma — `{3}{G}` 0/0 Legendary Creature —
//! Mutant Warrior.
//! Vigilance.
//! Lily Bowen enters with two +1/+1 counters on it.
//! At the beginning of your upkeep, double the number of +1/+1 counters on
//! Lily Bowen if its power is 16 or less. Otherwise, remove all but one
//! +1/+1 counter from it, then you gain 1 life for each +1/+1 counter
//! removed this way.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lily Bowen, Raging Grandma");
    let mutant = reg.interner_mut().intern("Mutant");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mutant);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Vigilance],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // Enters with two +1/+1 counters.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_two_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Upkeep: double counters if power <= 16, else remove all but one
            // and gain 1 life per removed.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_double_or_shed,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_two_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}

fn upkeep_double_or_shed(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let counters = state
        .objects
        .get(trig.source)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    let power = script::power_of(state, trig.source);
    if power <= 16 {
        // Double: add the same number of +1/+1 counters again.
        vec![Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: counters,
        }]
    } else {
        // Remove all but one, gain 1 life per removed.
        let removed = counters.saturating_sub(1);
        vec![
            Effect::RemoveCounters {
                target: trig.source,
                kind: CounterKind::PlusOnePlusOne,
                count: removed,
            },
            Effect::GainLife {
                player: trig.controller,
                amount: removed,
            },
        ]
    }
}
