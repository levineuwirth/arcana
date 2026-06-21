//! Savage Thallid — `{3}{G}{G}` 5/2 Fungus.
//!
//! At the beginning of your upkeep, put a spore counter on this creature.
//! Remove three spore counters from this creature: Create a 1/1 green
//!   Saproling creature token.
//! Sacrifice a Saproling: Regenerate target Fungus.
//!
//! All three abilities are expressible: the upkeep trigger adds a named
//! "spore" counter; the first activation pays three spore counters
//! (`remove_self_counter`) to make a Saproling; the second sacrifices a
//! chosen Saproling (`sacrifice_other`) to regenerate a target Fungus.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Savage Thallid");
    let fungus = reg.interner_mut().intern("Fungus");
    let spore = reg.interner_mut().intern("spore");
    let _saproling = reg.interner_mut().intern("Saproling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fungus);

    // Build the cost / target filters before the `reg.register` call so
    // we don't immutably borrow `reg` while it's borrowed mutably as the
    // method receiver.
    let sac_filter = script::subtype_filter(reg, "Saproling");
    let fungus_filter = script::subtype_filter(reg, "Fungus");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
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
                effect: add_spore_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove three spore counters from this creature: Create a 1/1 green Saproling creature token.".into(),
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
                text: "Sacrifice a Saproling: Regenerate target Fungus.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(sac_filter),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(fungus_filter),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: regenerate_fungus,
            }),
    )
}

fn add_spore_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(spore) = reg.interner().lookup("spore").map(CounterKind::Named) else {
        return Vec::new();
    };
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

fn regenerate_fungus(
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
    vec![Effect::Regenerate { target: *id }]
}
