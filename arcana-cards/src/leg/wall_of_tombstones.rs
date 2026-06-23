//! Wall of Tombstones — `{1}{B}` 0/1 Creature — Wall.
//!
//! Defender.
//! At the beginning of your upkeep, change this creature's base toughness to 1
//! plus the number of creature cards in your graveyard. (This effect lasts
//! indefinitely.)
//!
//! Defender is a base keyword. The upkeep trigger recomputes the base
//! toughness via `Effect::SetBasePT` with `Duration::Permanent` ("lasts
//! indefinitely"); the toughness amount is dynamic (1 + creature cards in
//! your graveyard), computed with `script::graveyard_matching`. Base power
//! stays 0.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wall of Tombstones");
    let wall = reg.interner_mut().intern("Wall");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wall);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Defender],
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
                effect: set_toughness_from_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Set base P/T to 0 / (1 + creature cards in your graveyard), indefinitely.
fn set_toughness_from_graveyard(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let creatures = script::graveyard_matching(
        state,
        &ObjectFilter::creature(),
        trig.controller,
        trig.controller,
    );
    let toughness = 1 + creatures as i32;
    vec![Effect::SetBasePT {
        target: trig.source,
        power: 0,
        toughness,
        duration: Duration::Permanent,
    }]
}
