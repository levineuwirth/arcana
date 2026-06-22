//! Ornery Tumblewagg — `{2}{G}` 2/2 green Brushwagg Mount.
//!
//! Oracle:
//! * "At the beginning of combat on your turn, put a +1/+1 counter on
//!   target creature." — a combat-begin trigger targeting a creature.
//! * "Whenever this creature attacks while saddled, double the number of
//!   +1/+1 counters on target creature." — a `SelfAttacks` trigger; the
//!   "while saddled" gate has no expressible saddled-state condition
//!   (GAP'd — the trigger fires on every attack). Doubling is modeled by
//!   reading the target's current +1/+1 count N and adding N more.
//! * Saddle 2 — Scryfall keyword `Saddle`. There is no
//!   `KeywordAbility::Saddle` variant and the tap-creatures saddle
//!   activation is not expressible. GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ornery Tumblewagg");
    let brushwagg = reg.interner_mut().intern("Brushwagg");
    let mount = reg.interner_mut().intern("Mount");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(brushwagg);
    subtypes.0.insert(mount);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Saddle 2 — no KeywordAbility::Saddle variant.
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: combat_add_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP: "while saddled" gate — no saddled-state condition.
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_double_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::target_creature()],
            }),
    )
}

fn combat_add_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

fn attack_double_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    // Double = add another N where N is the current +1/+1 counter count.
    let n = state
        .objects
        .get(*id)
        .map_or(0, |o| o.count_counters(CounterKind::PlusOnePlusOne));
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: n,
    }]
}
