//! Eluge, the Shoreless Sea — `{1}{U}{U}{U}` */* legendary Elemental Fish.
//! "Eluge's power and toughness are each equal to the number of Islands
//!  you control." (CDA — wired via a self_pt_from_match on Islands you
//!  control, installed on ETB.)
//! "Whenever Eluge enters or attacks, put a flood counter on target
//!  land. It's an Island in addition to its other types for as long as
//!  it has a flood counter on it." (counter placed; the conferred
//!  Island type is a GAP.)
//! "The first instant or sorcery spell you cast each turn costs less …"
//!  (cost reduction static — GAP'd.)

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Eluge, the Shoreless Sea");
    let elemental = reg.interner_mut().intern("Elemental");
    let fish = reg.interner_mut().intern("Fish");
    // pre-intern the flood counter name so the resolver can recover it.
    let _flood = reg.interner_mut().intern("flood");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elemental);
    subtypes.0.insert(fish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        // P/T are each equal to the number of Islands you control (CDA);
        // resolved at Layer 7a by the install_cda ETB trigger below.
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // "enters or attacks" → two triggers, same effect.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: flood_target_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![flood_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: flood_target_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![flood_target()],
            })
            // P/T = number of Islands you control (CDA).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: continuous "it's an Island while it has a flood counter".
        // GAP: first instant/sorcery you cast each turn costs less.
    )
}

fn install_cda(_s: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let filter =
        script::subtype_filter(reg, "Island").controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match(
            trig.source,
            filter,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn flood_target() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    }
}

fn flood_target_land(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let flood = reg
        .interner()
        .lookup("flood")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    vec![Effect::AddCounters {
        target: *id,
        kind: flood,
        count: 1,
    }]
}
