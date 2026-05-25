//! Wary Farmer — `{1}{G/W}{G/W}` 3/3 Kithkin Citizen.
//! Keywords: Surveil
//! "At the beginning of your end step, if another creature entered
//! the battlefield under your control this turn, surveil 1."
//!
//! GAP: "if another creature entered the battlefield under your
//! control this turn" — intervening-if tracking ETB events this
//! turn is not available via script helpers.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wary Farmer");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let citizen = reg.interner_mut().intern("Citizen");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(citizen);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G/W}{G/W}").expect("valid cost")),
        colors: ColorSet(ColorSet::GREEN | ColorSet::WHITE),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
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
                // GAP: intervening-if "if another creature entered this
                // turn" — no script helper for per-turn ETB tracking.
                intervening_if: None,
                effect: on_end_step,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn on_end_step(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Surveil { player: trig.controller, count: 1 }]
}
