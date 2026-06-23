//! Thallid Devourer — `{1}{G}{G}` 2/2 Fungus.
//!
//! Oracle:
//! At the beginning of your upkeep, put a spore counter on this creature.
//! Remove three spore counters from this creature: Create a 1/1 green
//!   Saproling creature token.
//! Sacrifice a Saproling: This creature gets +1/+2 until end of turn.
//!
//! Decomposition:
//! - Upkeep trigger → add a spore counter to self.
//! - Activated ability, cost "remove three spore counters" → create a 1/1
//!   green Saproling token.
//! - Activated ability, cost "sacrifice a Saproling" → +1/+2 until EOT.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::effects::TokenDefinition;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thallid Devourer");
    let fungus = reg.interner_mut().intern("Fungus");
    let _spore = reg.interner_mut().intern("spore");
    let _saproling = reg.interner_mut().intern("Saproling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);

    let spore = reg.interner().lookup("spore").expect("interned");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_spore,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove three spore counters from Thallid Devourer: Create a 1/1 green Saproling creature token.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Named(spore), 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: make_saproling,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a Saproling: Thallid Devourer gets +1/+2 until end of turn.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(script::subtype_filter_owned(reg)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            }),
    )
}

mod script {
    use super::*;
    pub fn subtype_filter_owned(reg: &CardRegistry) -> ObjectFilter {
        arcana_core::script::subtype_filter(reg, "Saproling")
    }
}

fn add_spore(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(spore) = reg.interner().lookup("spore") else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(spore),
        count: 1,
    }]
}

fn make_saproling(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let Some(saproling) = reg.interner().lookup("Saproling") else { return Vec::new(); };
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saproling);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: saproling,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn pump_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
