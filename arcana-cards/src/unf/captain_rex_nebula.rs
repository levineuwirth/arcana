//! Captain Rex Nebula — `{1}{R}{W}` 2/2 Legendary Creature — Human Pilot
//! Employee. "At the beginning of combat on your turn, choose target nonland
//! permanent you control. Until end of turn, it becomes a Vehicle artifact with
//! base power and toughness each equal to its mana value, and it gains crew 2
//! and 'Crash Land — Whenever this Vehicle deals damage, roll a six-sided die.
//! If the result is equal to this Vehicle's mana value, sacrifice this Vehicle,
//! then it deals that much damage to any target.'"
//!
//! Best-effort: the targeted combat trigger and the additive artifact-type
//! grant are expressible; the rest of the animation is not.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
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
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Captain Rex Nebula");
    let human = reg.interner_mut().intern("Human");
    let pilot = reg.interner_mut().intern("Pilot");
    let employee = reg.interner_mut().intern("Employee");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pilot);
    subtypes.0.insert(employee);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: animate_vehicle,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .without_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn animate_vehicle(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: cannot set base P/T equal to the target's mana value (no script
    // helper for "mana value of an object"), cannot add the Vehicle subtype,
    // cannot grant crew 2, and cannot grant the nested "Crash Land" triggered
    // ability. Best-effort: add the artifact type until end of turn.
    vec![Effect::AddType {
        target: *id,
        types: TypeLine::ARTIFACT.into(),
        duration: Duration::EndOfTurn,
    }]
}
