//! Lattice-Blade Mantis — `{3}{G}` 4/3 Phyrexian Insect.
//! "This creature enters with two oil counters on it."
//! "Whenever this creature attacks, you may remove an oil counter from
//! it. If you do, untap it and it gets +1/+1 until end of turn."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lattice-Blade Mantis");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let insect = reg.interner_mut().intern("Insect");
    let _oil = reg.interner_mut().intern("oil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(insect);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // "Enters with two oil counters" — modeled as an ETB trigger that adds
            // them (the replacement-effect form isn't a primitive here).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_oil_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: "you may remove an oil counter from it. If you do, untap it and
            // +1/+1." — a triggered ability has no remove-counter cost/gate path
            // ("if you do" depends on paying that cost); only activated abilities
            // carry remove_self_counter. Wired SelfAttacks with a no-op body.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: noop,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_oil_counters(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let oil = reg.interner().lookup("oil").map(CounterKind::Named);
    match oil {
        Some(kind) => vec![Effect::AddCounters {
            target: trig.source,
            kind,
            count: 2,
        }],
        None => Vec::new(),
    }
}

fn noop(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}
