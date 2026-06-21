//! Mine Layer — `{3}{R}` 1/1 red Dwarf.
//!
//! * {1}{R}, {T}: Put a mine counter on target land.
//! * Whenever a land with a mine counter on it becomes tapped, destroy it.
//! * When this creature leaves the battlefield, remove all mine counters
//!   from all lands.
//!
//! GAPs:
//! - Ability 2 effect: the "with a mine counter" land filter is not
//!   expressible (no counter predicate in the usable `ObjectFilter`
//!   refinements), and there is no accessor for the object that became
//!   tapped, so the destroy can't be targeted; GAP'd. (Trigger condition
//!   approximated as `BecomesTapped` over lands.)
//! - Ability 3: "leaves the battlefield" has no listed trigger variant;
//!   approximated with the closest, `SelfDies` (GAP: leaves-vs-dies). The
//!   remove-all-mine-counters-from-all-lands body IS wired via ForEach.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mine Layer");
    let dwarf = reg.interner_mut().intern("Dwarf");
    let _mine = reg.interner_mut().intern("mine");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dwarf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}, {T}: Put a mine counter on target land.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: put_mine_counter,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::BecomesTapped {
                    filter: ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
                },
                intervening_if: None,
                effect: destroy_mined_land_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: remove_all_mine_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn put_mine_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let Some(kind) = reg.interner().lookup("mine").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters { target: *id, kind, count: 1 }]
}

fn destroy_mined_land_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cannot filter "land with a mine counter" (no counter predicate)
    // nor identify the object that became tapped (no accessor).
    Vec::new()
}

fn remove_all_mine_counters(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("mine").map(CounterKind::Named) else {
        return Vec::new();
    };
    let lands = script::ids_matching(
        state,
        &ObjectFilter::permanent().with_types(TypeLine::LAND.into()),
        trig.controller,
    );
    // Remove up to a large count from each land (RemoveCounters caps at the
    // amount actually present).
    vec![Effect::ForEach {
        targets: lands,
        effect: Box::new(Effect::RemoveCounters {
            target: arcana_core::objects::NULL_OBJECT_ID,
            kind,
            count: 255,
        }),
    }]
}
