//! The Scorpion God — `{3}{B}{R}` 6/5 Legendary Creature — God.
//! "Whenever a creature with a -1/-1 counter on it dies, draw a card."
//! "{1}{B}{R}: Put a -1/-1 counter on another target creature."
//! "When The Scorpion God dies, return it to its owner's hand at the
//!  beginning of the next end step."
//!
//! Three abilities: a death-watch draw (filtered on the dying creature
//! carrying a -1/-1 counter), an activated -1/-1 counter placement, and a
//! self-death delayed return-to-hand at the next end step.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Scorpion God");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    let dying_creature = ObjectFilter {
        has_counter: Some(CounterKind::MinusOneMinusOne),
        ..ObjectFilter::creature()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: dying_creature,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: draw_on_death,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: schedule_return_to_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}{R}: Put a -1/-1 counter on another target creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_minus_counter,
            }),
    )
}

fn draw_on_death(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards {
        player: trig.controller,
        count: 1,
    }]
}

fn schedule_return_to_hand(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DelayedAction {
        source: trig.source,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::ReturnToHand,
    }]
}

fn put_minus_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "another target creature" — best-effort: the chosen target.
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::MinusOneMinusOne,
        count: 1,
    }]
}
