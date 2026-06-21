//! Marionette Master — `{4}{B}{B}` 1/3 Human Artificer.
//!
//! "Fabricate 3 (When this creature enters, put three +1/+1 counters on
//!  it or create three 1/1 colorless Servo artifact creature tokens.)
//!  Whenever an artifact you control is put into a graveyard from the
//!  battlefield, target opponent loses life equal to this creature's
//!  power."
//!
//! Fabricate is not in the supported keyword surface and its ETB is a
//! modal choice (counters OR tokens) that a `TriggeredAbilityDef` cannot
//! express, so that clause is GAP'd. The death-of-an-artifact trigger is
//! a faithful ZoneChange watcher: target opponent loses life equal to
//! this creature's power.

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Marionette Master");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: Fabricate 3 — keyword not in supported surface; its ETB is a
    // modal choice (three +1/+1 counters OR three Servo tokens) that a
    // TriggeredAbilityDef cannot express. Omitted.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .with_types(TypeLine::ARTIFACT.into())
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: opponent_loses_life_equal_to_power,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
            }),
    )
}

fn opponent_loses_life_equal_to_power(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Player(p) = target else { return Vec::new(); };
    let amount = script::power_of(state, trig.source).max(0) as u32;
    vec![Effect::LoseLife { player: *p, amount }]
}
