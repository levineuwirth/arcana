//! Psychomancer — `{1}{B}` 1/1 Artifact Creature — Necron Wizard with
//! Flying.
//! "Harbinger of Despair — Whenever this creature or another nontoken
//! artifact you control is put into a graveyard from the battlefield or
//! is put into exile from the battlefield, target opponent loses 1 life
//! and you gain 1 life."
//!
//! Flying is a base keyword. The Harbinger trigger watches nontoken
//! artifacts you control leaving the battlefield. A single
//! `ZoneChange` carries one destination, so it is split into two
//! triggers — one for battlefield→graveyard and one for
//! battlefield→exile — both with the same effect (target opponent
//! loses 1 life; you gain 1 life).

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
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

fn nontoken_artifact_filter() -> ObjectFilter {
    ObjectFilter::new()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You)
        .nontoken()
}

fn target_opponent_req() -> TargetRequirement {
    TargetRequirement {
        filter: TargetFilter::Player,
        count: TargetCount::Exactly(1),
        controller: Some(ControllerConstraint::Opponent),
    }
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Psychomancer");
    let necron = reg.interner_mut().intern("Necron");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(necron);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: nontoken_artifact_filter(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: drain_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target_opponent_req()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: nontoken_artifact_filter(),
                    from: Some(Zone::Battlefield),
                    to: Zone::Exile,
                },
                intervening_if: None,
                effect: drain_one,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![target_opponent_req()],
            }),
    )
}

fn drain_one(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(opp)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::LoseLife { player: *opp, amount: 1 },
        Effect::GainLife { player: trig.controller, amount: 1 },
    ]
}
