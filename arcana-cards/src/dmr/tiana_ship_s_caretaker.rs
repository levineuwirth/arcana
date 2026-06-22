//! Tiana, Ship's Caretaker — `{3}{R}{W}` 3/3 Legendary Angel Artificer with
//! Flying and First strike.
//!
//! Oracle:
//! * Flying, first strike (keywords).
//! * Whenever an Aura or Equipment you control is put into a graveyard from
//!   the battlefield, you may return that card to its owner's hand at the
//!   beginning of the next end step.
//!
//! The triggered ability watches Aura/Equipment permanents you control going
//! to the graveyard (a `ZoneChange` from battlefield → graveyard) and schedules
//! a delayed `ReturnToHand` for that card at the next end step. The "you may"
//! optionality is a resolution-time choice not separately modeled — the delayed
//! return is scheduled unconditionally (a documented fidelity gap).

use arcana_core::effects::{Effect, DelayedWhen, DelayedAction, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tiana, Ship's Caretaker");
    let angel = reg.interner_mut().intern("Angel");
    let artificer = reg.interner_mut().intern("Artificer");
    let aura = reg.interner_mut().intern("Aura");
    let equipment = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(angel);
    subtypes.0.insert(artificer);

    // Filter: an Aura or Equipment you control (subtype OR).
    let aura_or_equipment = ObjectFilter::new()
        .with_subtypes_any(vec![aura, equipment])
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::ZoneChange {
                filter: aura_or_equipment,
                from: Some(Zone::Battlefield),
                to: Zone::Graveyard(0),
            },
            intervening_if: None,
            effect: schedule_return,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn schedule_return(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Schedule the dying Aura/Equipment card to return to its owner's hand at
    // the next end step. ("You may" optionality is not separately modeled.)
    let Some(card) = trig.dying_object() else {
        return Vec::new();
    };
    vec![Effect::DelayedAction {
        source: card,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::ReturnToHand,
    }]
}
