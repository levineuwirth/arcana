//! Bringer of the White Dawn — `{7}{W}{W}` 5/5 Bringer.
//! * "You may pay {W}{U}{B}{R}{G} rather than pay this spell's mana cost."
//!   (alternative cost; GAP)
//! * Trample (keyword)
//! * "At the beginning of your upkeep, you may return target artifact card from
//!   your graveyard to the battlefield." (targeted upkeep trigger)
//!
//! GAP: the rainbow alternative cost is a cast-time payment alternative with no
//! demonstrated API surface (only the printed mana cost is modeled).

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bringer of the White Dawn");
    let bringer = reg.interner_mut().intern("Bringer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bringer);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    let artifact_in_gy = TargetFilter::Card {
        zone: Zone::Graveyard(0),
        filter: ObjectFilter::new().with_types(TypeLine::ARTIFACT.into()),
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
                effect: return_artifact,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: artifact_in_gy,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            }),
    )
}

fn return_artifact(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToBattlefield { target: *id }]
}
