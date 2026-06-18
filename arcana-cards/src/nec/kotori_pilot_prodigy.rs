//! Kotori, Pilot Prodigy — `{1}{W}{U}` Legendary 2/4 Moonfolk Pilot.
//!
//! Oracle:
//! * "Vehicles you control have crew 2." — GAP: a pure static ability
//!   that REDUCES the crew cost of other Vehicles you control. There is
//!   no demonstrated effect/static primitive that rewrites a Vehicle's
//!   crew cost, so this static is omitted.
//! * "At the beginning of combat on your turn, target artifact creature
//!   you control gains lifelink and vigilance until end of turn." →
//!   `PhaseBegins { Combat, You }` trigger targeting an artifact creature
//!   you control, granting Lifelink and Vigilance until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
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
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kotori, Pilot Prodigy");
    let moonfolk = reg.interner_mut().intern("Moonfolk");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(moonfolk);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    // GAP: "Vehicles you control have crew 2." — static crew-cost
    // reduction; no demonstrated primitive expresses it.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: grant_lifelink_vigilance,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE))
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn grant_lifelink_vigilance(
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
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Lifelink,
            duration: Duration::EndOfTurn,
        },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Vigilance,
            duration: Duration::EndOfTurn,
        },
    ]
}
