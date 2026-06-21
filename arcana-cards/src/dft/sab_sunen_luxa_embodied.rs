//! Sab-Sunen, Luxa Embodied — `{3}{G}{U}` 6/6 Legendary Creature — God.
//! Reach, trample, indestructible. "Can't attack or block unless it has an
//! even number of counters on it." "At the beginning of your first main
//! phase, put a +1/+1 counter on Sab-Sunen. Then if it has an odd number of
//! counters on it, draw two cards."

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sab-Sunen, Luxa Embodied");
    let god = reg.interner_mut().intern("God");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    // GAP: static "can't attack or block unless it has an even number of
    // counters on it" — no modeled primitive for parity-gated attack/block.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        keywords: vec![
            KeywordAbility::Reach,
            KeywordAbility::Trample,
            KeywordAbility::Indestructible,
        ],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::PreCombatMain,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: main_phase_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn main_phase_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Then if it has an odd number of counters on it, draw two cards"
    // — parity-conditional follow-up is not expressible with the documented
    // Condition/script helpers; the counter half is emitted.
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
