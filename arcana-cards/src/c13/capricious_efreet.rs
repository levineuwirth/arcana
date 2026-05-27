//! Capricious Efreet — `{4}{R}{R}` 6/4 red Efreet.
//! "At the beginning of your upkeep, choose target nonland permanent you control and up to two
//! target nonland permanents you don't control. Destroy one of them at random."
//! GAP: "destroy one of them at random" — random selection among targets not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectFilter};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Capricious Efreet");
    let efreet = reg.interner_mut().intern("Efreet");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(efreet);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(6)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: |_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry| {
                    // GAP: "destroy one of them at random" — random selection among targets not expressible
                    // Destroying first opponent's target as best-effort
                    let opponent_target = trig.targets.targets.iter()
                        .find(|t| matches!(t, TargetChoice::Object(_)));
                    if let Some(TargetChoice::Object(id)) = opponent_target {
                        vec![Effect::DestroyPermanent { target: *id }]
                    } else {
                        Vec::new()
                    }
                },
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::new().without_types(TypeLine::LAND.into())),
                        count: TargetCount::Exactly(1),
                        controller: Some(ControllerConstraint::You),
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(ObjectFilter::new().without_types(TypeLine::LAND.into())),
                        count: TargetCount::UpTo(2),
                        controller: Some(ControllerConstraint::Opponent),
                    },
                ],
            }),
    )
}
