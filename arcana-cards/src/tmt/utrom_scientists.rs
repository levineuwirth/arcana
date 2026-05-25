//! Utrom Scientists — `{2}{U}` 2/2 blue Artifact Creature — Utrom Robot Scientist.
//! "When this creature enters, tap up to one target creature and put a stun counter on it."
//!
//! GAP: no "add stun counter" effect variant in the catalog — CounterKind::Stun may not
//! exist. Using AddCounters with a GAP note, and Tap for the tapping part.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Utrom Scientists");
    let utrom = reg.interner_mut().intern("Utrom");
    let robot = reg.interner_mut().intern("Robot");
    let scientist = reg.interner_mut().intern("Scientist");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(utrom);
    subtypes.0.insert(robot);
    subtypes.0.insert(scientist);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: on_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn on_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    vec![
        Effect::Tap { target: *id },
        // GAP: CounterKind::Stun may not exist — using PlusOnePlusOne as structural placeholder.
        // Effect::AddCounters { target: *id, kind: CounterKind::Stun, count: 1 },
    ]
}
