//! Workshop Elders — `{6}{U}` 4/4 blue Human Artificer.
//!
//! Artifact creatures you control have flying.
//! At the beginning of combat on your turn, you may have target
//! noncreature artifact you control become a 0/0 artifact creature. If
//! you do, put four +1/+1 counters on it.
//!
//! The static "artifact creatures you control have flying" anthem is
//! GAP'd (ability-granting continuous static, no shape). The
//! combat-trigger animation IS expressed: the chosen noncreature
//! artifact gains the creature type and base 0/0, then gets four +1/+1
//! counters. The "you may" is approximated as a mandatory targeted
//! effect (a minor fidelity simplification).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Workshop Elders");
    let human = reg.interner_mut().intern("Human");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(artificer);

    let noncreature_artifact = ObjectFilter::permanent()
        .with_types(TypeLine::ARTIFACT.into())
        .without_types(TypeLine::CREATURE.into())
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: static "Artifact creatures you control have flying" — anthem-
    // style ability-grant continuous static.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::PhaseBegins {
                phase: Phase::Combat,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: animate_artifact,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(noncreature_artifact),
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn animate_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::AddType {
            target: *id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::Permanent,
        },
        Effect::SetBasePT {
            target: *id,
            power: 0,
            toughness: 0,
            duration: Duration::Permanent,
        },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::PlusOnePlusOne,
            count: 4,
        },
    ]
}
