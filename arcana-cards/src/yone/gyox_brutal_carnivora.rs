//! Gyox, Brutal Carnivora — `{1}{B}{G}` 2/4 Legendary Creature — Phyrexian
//! Warlock.
//!
//! Oracle:
//! * At the beginning of your end step, put an oil counter on up to one
//!   target creature. (Triggered, targeted.)
//! * Whenever a nontoken creature you control with one or more oil
//!   counters on it dies, conjure X duplicates of it into exile, where X
//!   is the number of oil counters on it. Those duplicates perpetually
//!   get +X/+X. Then shuffle those duplicates into your library.
//!   (GAP — Conjure is an Alchemy-only mechanic with no Effect variant.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gyox, Brutal Carnivora");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let warlock = reg.interner_mut().intern("Warlock");
    let oil = reg.interner_mut().intern("oil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    let _ = oil; // counter kind built via Named in the resolver

    reg.register(
        CardDefinition::new(name, chars)
            // At the beginning of your end step, put an oil counter on up to
            // one target creature.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: end_step_oil_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            // GAP: "Whenever a nontoken creature you control with one or more
            // oil counters on it dies, conjure X duplicates of it ..." —
            // Conjure (Alchemy-only) is not modeled; there is no Effect::Conjure
            // variant and the perpetual buff + shuffle-into-library has no API.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You)
                        .nontoken(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: conjure_duplicates_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn end_step_oil_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    let Some(oil) = reg.interner().lookup("oil") else { return Vec::new(); };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::Named(oil),
        count: 1,
    }]
}

fn conjure_duplicates_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Conjure X duplicates into exile, perpetual +X/+X, then shuffle into
    // library — Conjure is Arena/Alchemy-only with no Effect variant.
    Vec::new()
}
