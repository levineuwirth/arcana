//! Necrosquito — `{3}{B}` 0/0 Phyrexian Insect with Flying.
//! "This creature enters with two oil counters on it. This creature gets
//! +1/+1 for each oil counter on it. Whenever another creature or
//! artifact you control is put into a graveyard from the battlefield, put
//! an oil counter on this creature."
//!
//! Keyword line: Flying. "Enters with two oil counters" is modeled as an
//! ETB trigger that adds two oil counters (a named counter) to itself —
//! the closest expressible shape for an enters-with-counters replacement.
//! The "+1/+1 for each oil counter" line is a continuous characteristic-
//! defining static (no enters-with/CDA-from-counters expressible here) —
//! GAP. The death-watch trigger adds an oil counter whenever another
//! creature/artifact you control hits the graveyard from the battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Necrosquito");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let insect = reg.interner_mut().intern("Insect");
    let _oil = reg.interner_mut().intern("oil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(insect);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        keywords: vec![KeywordAbility::Flying],
        // GAP: static — "gets +1/+1 for each oil counter on it" (continuous CDA from counters, not expressible here).
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_two_oil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::permanent()
                        .controlled_by(ControllerConstraint::You)
                        .with_types_any(TypeLine(TypeLine::CREATURE | TypeLine::ARTIFACT)),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: add_one_oil,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_two_oil(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(oil) = reg.interner().lookup("oil").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters { target: trig.source, kind: oil, count: 2 }]
}

fn add_one_oil(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(oil) = reg.interner().lookup("oil").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters { target: trig.source, kind: oil, count: 1 }]
}
