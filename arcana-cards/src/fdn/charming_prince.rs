//! Charming Prince — `{1}{W}` 2/2 Human Noble.
//!
//! When this creature enters, choose one —
//! • Scry 2.
//! • You gain 3 life.
//! • Exile another target creature you own. Return it to the battlefield
//!   under your control at the beginning of the next end step.

use arcana_core::effects::{Effect, DelayedAction, DelayedWhen};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerMode,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Charming Prince");
    let human = reg.interner_mut().intern("Human");
    let noble = reg.interner_mut().intern("Noble");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(noble);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_modal,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }).with_trigger_modes(1, vec![
            TriggerMode {
                description: "Scry 2".into(),
                target_requirements: Vec::new(),
                effect: scry,
            },
            TriggerMode {
                description: "Gain 3 life".into(),
                target_requirements: Vec::new(),
                effect: gain_life,
            },
            TriggerMode {
                description: "Exile another creature you own; return it at the next end step".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter::creature().excluding_source())
                        .owned_by(ControllerConstraint::You),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                effect: blink,
            },
        ]),
    )
}

// The engine dispatches the chosen clause; this ordinary callback is unused.
fn etb_modal(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    Vec::new()
}

fn scry(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Scry { player: trig.controller, count: 2 }]
}

fn gain_life(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: trig.controller, amount: 3 }]
}

fn blink(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    match trig.targets.targets.first() {
        Some(TargetChoice::Object(target)) => vec![Effect::ExileAndSchedule {
            target: *target, source: trig.source, controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileUnderYourControl,
        }],
        _ => Vec::new(),
    }
}
