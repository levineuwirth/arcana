//! Mistmeadow Vanisher — `{2}{W/U}` 3/2 blue-white creature (Kithkin
//! Wizard). "Whenever this creature becomes tapped, exile up to one
//! target nonland, nontoken permanent. Return that card to the
//! battlefield under its owner's control at the beginning of the next
//! end step."
//!
//! GAP: "return to battlefield under its owner's control at beginning
//! of next end step" — ReturnFromExileToBattlefield exists but the
//! delayed scheduling to end step is approximated via DelayedAction.
//! Owner rather than controller is not distinguishable.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
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
    let name = reg.interner_mut().intern("Mistmeadow Vanisher");
    let kithkin = reg.interner_mut().intern("Kithkin");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(kithkin);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W/U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesTapped,
                intervening_if: None,
                effect: on_tapped,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::default()
                            .without_types(TypeLine::LAND.into())
                            .nontoken(),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn on_tapped(
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
        Effect::ExilePermanent { target: *id },
        Effect::DelayedAction {
            source: *id,
            controller: trig.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::ReturnFromExileToBattlefield,
        },
    ]
}
