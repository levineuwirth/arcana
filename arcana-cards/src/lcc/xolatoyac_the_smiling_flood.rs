//! Xolatoyac, the Smiling Flood — `{4}{G}{U}` 6/6 Legendary Salamander Serpent.
//! "Whenever Xolatoyac enters or attacks, put a flood counter on target land.
//! That land is an Island in addition to its other types for as long as it has
//! a flood counter on it." and "At the beginning of your end step, untap each
//! permanent you control with a counter on it."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

fn land_target() -> Vec<TargetRequirement> {
    vec![TargetRequirement {
        filter: TargetFilter::Permanent(
            ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        ),
        count: TargetCount::Exactly(1),
        controller: None,
    }]
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Xolatoyac, the Smiling Flood");
    let salamander = reg.interner_mut().intern("Salamander");
    let serpent = reg.interner_mut().intern("Serpent");
    let _flood = reg.interner_mut().intern("flood");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(salamander);
    subtypes.0.insert(serpent);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: flood_counter_on_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: land_target(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: flood_counter_on_land,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: land_target(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: untap_countered_permanents,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn flood_counter_on_land(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let flood = reg.interner().lookup("flood").unwrap_or_default();
    // GAP: "is an Island for as long as it has a flood counter" — a counter-
    // conditional type-grant duration is not expressible; only the counter is
    // applied here.
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(flood),
        count: 1,
    }]
}

fn untap_countered_permanents(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "with a counter on it" approximated as +1/+1 counters (no "any counter"
    // ObjectFilter predicate; has_counter is keyed to a single kind).
    let filter = ObjectFilter {
        has_counter: Some(CounterKind::PlusOnePlusOne),
        ..ObjectFilter::permanent().controlled_by(ControllerConstraint::You)
    };
    let ids = script::ids_matching(state, &filter, trig.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Untap { target: NULL_OBJECT_ID }),
    }]
}
