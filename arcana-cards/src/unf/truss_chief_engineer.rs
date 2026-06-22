//! Truss, Chief Engineer — `{U}{B}` 1/3 Legendary Vedalken Rogue
//! Employee.
//!
//! Whenever Truss enters or another creature dies, put a hack counter
//!   on Truss.
//! {2}, {T}, Remove X hack counters from Truss: Add or subtract X from
//!   a number or number word on target spell or permanent until end of
//!   turn. (This effect can't reduce a number below 1 …)
//!
//! The "enters OR another creature dies" line decomposes into two
//! triggered abilities (a self-ETB and a creature-death `ZoneChange`),
//! each adding a hack counter to Truss. The activated ability is GAP'd
//! in full: its cost removes a VARIABLE X hack counters (not a fixed
//! count, which `remove_self_counter` requires) and its effect mutates
//! a number / number word on a target — neither is expressible.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Truss, Chief Engineer");
    let vedalken = reg.interner_mut().intern("Vedalken");
    let rogue = reg.interner_mut().intern("Rogue");
    let employee = reg.interner_mut().intern("Employee");
    // pre-intern the named counter so it can be recovered in the effect fn
    let _hack = reg.interner_mut().intern("hack");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vedalken);
    subtypes.0.insert(rogue);
    subtypes.0.insert(employee);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: "{2}, {T}, Remove X hack counters from Truss: Add or
    // subtract X from a number or number word on target spell or
    // permanent until end of turn." — variable-X counter removal and
    // number-word mutation are unexpressible; the whole activated
    // ability is omitted.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: add_hack_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Any),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: add_hack_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_hack_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(hack) = reg.interner().lookup("hack") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(hack),
        count: 1,
    }]
}
