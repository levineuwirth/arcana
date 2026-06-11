//! Bounty of the Luxa — `{2}{G}{U}` enchantment.
//! "At the beginning of your first main phase, remove all flood
//! counters from this enchantment. If no counters were removed this
//! way, put a flood counter on this enchantment and draw a card.
//! Otherwise, add {C}{G}{U}."
//!
//! The alternation is resolved by checking for a flood counter on the
//! source at resolution (`conditions::source_has_counter`): only this
//! card places flood counters and always exactly one, so "remove all"
//! is modeled as remove one.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bounty of the Luxa");
    let _flood = reg.interner_mut().intern("flood");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: luxa_bounty,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// Remove flood counters; none removed → counter + draw, else {C}{G}{U}.
fn luxa_bounty(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let flood = reg.interner().lookup("flood").unwrap_or_default();
    let kind = CounterKind::Named(flood);
    // GAP: fidelity — "remove ALL flood counters" is modeled as remove
    // one (only this card adds flood counters, one at a time).
    if conditions::source_has_counter(state, trig.source, kind.clone()) {
        vec![
            Effect::RemoveCounters {
                target: trig.source,
                kind,
                count: 1,
            },
            Effect::AddMana {
                player: trig.controller,
                mana: vec![
                    ManaUnit::plain(ManaColor::Colorless, trig.source),
                    ManaUnit::plain(ManaColor::Green, trig.source),
                    ManaUnit::plain(ManaColor::Blue, trig.source),
                ],
            },
        ]
    } else {
        vec![
            Effect::AddCounters {
                target: trig.source,
                kind,
                count: 1,
            },
            Effect::DrawCards {
                player: trig.controller,
                count: 1,
            },
        ]
    }
}
