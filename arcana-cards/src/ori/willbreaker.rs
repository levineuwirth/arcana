//! Willbreaker — `{3}{U}{U}` 2/3 blue Human Wizard.
//! "Whenever a creature an opponent controls becomes the target of a spell or
//! ability you control, gain control of that creature for as long as you
//! control Willbreaker."
//!
//! GAP: trigger — no TriggerCondition variant for "whenever a creature an
//! opponent controls becomes the target of a spell or ability you control".
//! SelfBecomesTarget only watches THIS creature. A new board-wide variant
//! is needed. Using the closest structural proxy (ZoneChange) which will
//! not fire correctly — the effect fn body is also incomplete since
//! ChangeControl with a timed duration is not supported.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Willbreaker");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    // GAP: trigger — "whenever a creature an opponent controls becomes the
    // target of a spell or ability you control" has no matching TriggerCondition.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: gain_control_of_creature,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn gain_control_of_creature(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: effect should be "gain control for as long as you control Willbreaker"
    // — ChangeControl is permanent, not duration-scoped to source presence.
    vec![Effect::ChangeControl { target: *id, new_controller: trig.controller }]
}
