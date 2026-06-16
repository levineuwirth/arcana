//! Clockwork Steed — `{4}` 0/3 colorless Artifact Creature — Horse.
//! "This creature enters with four +1/+0 counters on it." — +1/+0 has no dedicated
//!  CounterKind, so a Named("+1/+0") counter is used; "enters with" is modeled as an
//!  ETB trigger that adds the counters (documented partial vs a true replacement).
//! "This creature can't be blocked by artifact creatures." — filtered can't-be-blocked
//!  (only by a creature type) is not expressible; GAP'd.
//! "At end of combat, if this creature attacked or blocked this combat, remove a
//!  +1/+0 counter from it." — the attacked-or-blocked intervening-if has no helper;
//!  left None (over-fires), counter removal emitted.
//! "{X}, {T}: Put up to X +1/+0 counters on this creature... can't exceed four.
//!  Activate only during your upkeep." — X-cost + upkeep-only timing + the four-cap
//!  rider are not expressible; the ability is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Clockwork Steed");
    let horse = reg.interner_mut().intern("Horse");
    let _plus = reg.interner_mut().intern("+1/+0");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "can't be blocked by artifact creatures" — filtered (by-type) evasion not expressible.
    // GAP: "{X}, {T}: Put up to X +1/+0 counters (cap 4), only during your upkeep" — X-cost,
    //   timing restriction, and the cap rider are not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_add_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                // GAP (intervening-if): "if this creature attacked or blocked this combat"
                // has no conditions:: helper; left None (over-fires every end of combat).
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::EndCombat,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: None,
                effect: end_combat_remove_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_add_counters(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(plus) = reg.interner().lookup("+1/+0") else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(plus),
        count: 4,
    }]
}

fn end_combat_remove_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(plus) = reg.interner().lookup("+1/+0") else { return Vec::new(); };
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind: CounterKind::Named(plus),
        count: 1,
    }]
}
