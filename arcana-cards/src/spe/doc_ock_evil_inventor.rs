//! Doc Ock, Evil Inventor — `{5}{U}{B}` 8/8 blue/black Legendary Human
//! Scientist Villain.
//! "At the beginning of combat on your turn, target noncreature artifact
//! you control becomes an 8/8 Robot Villain artifact creature in addition
//! to its other types."
//! The animation is the additive Layer-4 `Effect::AddType` (Creature)
//! plus `Effect::SetBasePT` 8/8 plus a targeted Robot+Villain
//! subtype-add continuous effect (`ContinuousEffect::add_subtypes`),
//! all `Duration::Permanent` (the oracle text has no duration).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doc Ock, Evil Inventor");
    let human = reg.interner_mut().intern("Human");
    let scientist = reg.interner_mut().intern("Scientist");
    let villain = reg.interner_mut().intern("Villain");
    let _robot = reg.interner_mut().intern("Robot"); // looked up in animate_artifact
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(scientist);
    subtypes.0.insert(villain);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
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
                effect: animate_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::ARTIFACT.into())
                            .without_types(TypeLine::CREATURE.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn animate_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // "becomes an 8/8 Robot Villain artifact creature in addition to
    // its other types" — additive Layer-4 type overlay plus base P/T
    // plus a targeted Robot+Villain subtype-add, all permanent (the
    // oracle text has no duration).
    let mut subs = SubtypeSet::default();
    for sub in ["Robot", "Villain"] {
        if let Some(s) = reg.interner().lookup(sub) {
            subs.0.insert(s);
        }
    }
    vec![
        Effect::AddType {
            target: *id,
            types: TypeLine::CREATURE.into(),
            duration: Duration::Permanent,
        },
        Effect::SetBasePT {
            target: *id,
            power: 8,
            toughness: 8,
            duration: Duration::Permanent,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::add_subtypes(trig.source, *id, subs, Duration::Permanent),
        },
    ]
}
