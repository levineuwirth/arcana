//! Erudite Wizard — `{2}{U}` 2/3 blue Human Wizard.
//! "Whenever you draw your second card each turn, put a +1/+1 counter on this
//! creature."
//!
//! "your second card each turn": CardDrawn fires on every draw (EachTime); an
//! intervening_if of `cards_drawn_this_turn(...) == 2` narrows it to exactly
//! the second draw — the triggering draw is already in the event log at
//! intervening-if time (same pattern as dds/jori_en_ruin_diver.rs for spells).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Erudite Wizard");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            // "your second card each turn": CardDrawn fires on every draw;
            // the intervening_if narrows it to exactly the second one.
            trigger_condition: TriggerCondition::CardDrawn {
                player: ControllerConstraint::You,
            },
            intervening_if: Some(iif_second_draw),
            effect: on_draw_add_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

/// True exactly when the triggering draw is your second card this turn.
/// intervening_if runs at trigger stack-add time, when the DrawCard event
/// that fired the trigger is ALREADY in the event log — so the count is 2
/// exactly on the second draw (1 on the first, 3+ afterwards).
fn iif_second_draw(state: &GameState, _source: ObjectId, you: PlayerId, _reg: &CardRegistry) -> bool {
    arcana_core::script::cards_drawn_this_turn(state, you) == 2
}

fn on_draw_add_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
