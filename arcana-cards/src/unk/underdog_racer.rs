//! Underdog Racer — `{4}{G}` 5/5 Dog Pilot.
//! This spell costs {1} less to cast if your team is currently behind. (GAP:
//! conditional cost reduction is not expressible.)
//! When this creature enters, destroy target artifact an opponent controls.

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Underdog Racer");
    let dog = reg.interner_mut().intern("Dog");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dog);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: static — "costs {1} less to cast if your team is currently behind"
    // is a conditional cost reduction with no expressible mechanism.

    let artifact_opp = ObjectFilter::new()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::Opponent);

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_destroy_artifact,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Permanent(artifact_opp),
                count: TargetCount::Exactly(1),
                controller: None,
            }],
        }),
    )
}

fn etb_destroy_artifact(
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
    vec![Effect::DestroyPermanent { target: *id }]
}
