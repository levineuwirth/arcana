//! A-Moss-Pit Skeleton — `{B}{G}` 2/2 Plant Skeleton with Kicker {3}.
//! "If Moss-Pit Skeleton was kicked, it enters with three +1/+1 counters on it.
//! Whenever one or more +1/+1 counters are put on a creature you control, if
//! Moss-Pit Skeleton is in your graveyard, return A-Moss-Pit Skeleton from your
//! graveyard to your hand at the beginning of the next end step."
//!
//! Kicker is not in the permitted keyword surface, and the "if it was kicked, enters
//! with three counters" replacement is GAP'd (no kicked predicate / ETB-counters
//! replacement). The recursion trigger (counters on a creature you control ->
//! return self to hand) is modeled; its "if Moss-Pit Skeleton is in your graveyard"
//! intervening-if is GAP'd (no in-own-graveyard predicate), and the trigger watches
//! from the graveyard zone.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Moss-Pit Skeleton");
    let plant = reg.interner_mut().intern("Plant");
    let skeleton = reg.interner_mut().intern("Skeleton");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(plant);
    subtypes.0.insert(skeleton);

    let your_creature = ObjectFilter::creature().controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        // GAP: Kicker {3} — not in the permitted keyword surface for this class.
        ..Default::default()
    };

    // GAP: "If it was kicked, it enters with three +1/+1 counters" — no kicked
    //      predicate / ETB-counters replacement effect.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::CounterAdded {
                on: TriggerSelf::AnyMatching(your_creature),
                kind: Some(CounterKind::PlusOnePlusOne),
                chapter: None,
            },
            intervening_if: None, // GAP: "if Moss-Pit Skeleton is in your graveyard".
            effect: schedule_return,
            trigger_zones: vec![Zone::Graveyard(0)],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn schedule_return(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DelayedAction {
        source: trig.source,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::ReturnToHand,
    }]
}
