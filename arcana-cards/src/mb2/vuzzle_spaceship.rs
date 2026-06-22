//! Vuzzle Spaceship — `{4}{R}{R}` 0/0 Artifact Creature — Construct.
//! Spaceship 6 (enters with six +1/+1 counters; damage removes that many
//! +1/+1 counters instead; abilities active at that counter threshold).
//! Thrusters [6] → Flying. (GAP — counter-threshold keyword grant.)
//! Lasers [3] → "Whenever Vuzzle Spaceship attacks, it deals 1 damage to
//! any target."
//!
//! Spaceship is not a modeled keyword. Its enters-with-six-counters part
//! is wired as an ETB trigger; the damage-removes-counters replacement is
//! a GAP. Thrusters (Flying while ≥6 counters) is a counter-gated static
//! keyword grant with no install hook, so it is GAP'd. Lasers is wired:
//! the attack trigger is gated by an intervening-if requiring at least 3
//! +1/+1 counters, and deals 1 damage to any target.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vuzzle Spaceship");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };

    // GAP: Spaceship's damage-removes-+1/+1-counters replacement effect.
    // GAP: Thrusters [6] → Flying — counter-threshold-gated static keyword
    // grant, no install hook.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: enter_with_six_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: Some(if_lasers_online),
                effect: lasers_ping,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }),
    )
}

fn enter_with_six_counters(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 6,
    }]
}

fn if_lasers_online(
    s: &GameState,
    source: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::source_counters_at_least(s, source, CounterKind::PlusOnePlusOne, 3)
}

fn lasers_ping(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: trig.source,
        target: dt,
        amount: 1,
    }]
}
