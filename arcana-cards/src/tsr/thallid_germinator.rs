//! Thallid Germinator — `{2}{G}` 2/2 Fungus.
//! "At the beginning of your upkeep, put a spore counter on this
//!  creature.
//!  Remove three spore counters from this creature: Create a 1/1 green
//!  Saproling creature token.
//!  Sacrifice a Saproling: Target creature gets +1/+1 until end of turn."
//!
//! Upkeep trigger adds a spore counter (named counter). First activated
//! ability removes three spore counters as a cost and mints a Saproling.
//! Second activated ability sacrifices a chosen Saproling you control
//! (sacrifice_other with a Saproling subtype filter) to pump a target
//! creature +1/+1 until end of turn.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thallid Germinator");
    let fungus = reg.interner_mut().intern("Fungus");
    // Pre-intern the spore counter name + Saproling subtype so both the
    // declarative cost and the effect fns can use them.
    let spore = reg.interner_mut().intern("spore");
    let saproling = reg.interner_mut().intern("Saproling");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    let def = CardDefinition::new(name, chars)
        .with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::Upkeep,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: upkeep_add_spore,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        })
        .with_activated_ability(ActivatedAbilityDef {
            text: "Remove three spore counters from Thallid Germinator: \
                   Create a 1/1 green Saproling creature token."
                .into(),
            cost: ActivationCost {
                remove_self_counter: Some((CounterKind::Named(spore), 3)),
                ..ActivationCost::default()
            },
            target_requirements: vec![],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: make_saproling,
        })
        .with_activated_ability(ActivatedAbilityDef {
            text: "Sacrifice a Saproling: Target creature gets +1/+1 \
                   until end of turn."
                .into(),
            cost: ActivationCost {
                sacrifice_other: Some(ObjectFilter {
                    types: Some(TypeLine::CREATURE.into()),
                    subtypes: Some(vec![saproling]),
                    ..ObjectFilter::default()
                }),
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement::target_creature()],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_target,
        });

    reg.register(def)
}

fn upkeep_add_spore(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let spore = reg
        .interner()
        .lookup("spore")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    vec![Effect::AddCounters {
        target: trig.source,
        kind: spore,
        count: 1,
    }]
}

fn make_saproling(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let saproling = reg.interner().lookup("Saproling").unwrap_or_default();
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

fn pump_target(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
