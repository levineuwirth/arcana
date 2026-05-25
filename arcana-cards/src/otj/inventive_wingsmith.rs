//! Inventive Wingsmith — `{2}{W}` 2/4 white Dwarf Artificer creature.
//! "At the beginning of your end step, if you haven't cast a spell from your hand this turn
//! and this creature doesn't have a flying counter on it, put a flying counter on it."
//!
//! # Notes
//! GAP: "flying counter" — CounterKind::Flying not in catalog; put +1/+1 counter approximation.
//! GAP: "if you haven't cast a spell from your hand this turn" — intervening_if not expressible.

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Inventive Wingsmith");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                // GAP: intervening_if — "if you haven't cast a spell from hand this turn
                // and this doesn't have a flying counter" not expressible.
                intervening_if: None,
                effect: end_step_flying_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_flying_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: CounterKind::Flying not in catalog — using PlusOnePlusOne as placeholder.
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::PlusOnePlusOne, count: 1 }]
}
