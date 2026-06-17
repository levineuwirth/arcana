//! Osseous Sticktwister — `{1}{B}` 2/2 Artifact Creature — Scarecrow.
//! Lifelink.
//!
//! Delirium — At the beginning of your end step, if there are four or
//! more card types among cards in your graveyard, each opponent may
//! sacrifice a nonland permanent or discard a card; then this creature
//! deals damage equal to its power to each opponent who didn't.
//!
//! The Delirium intervening-if ("four or more card types among cards in
//! your graveyard") has no demonstrated predicate, and the effect's
//! choose-then-conditionally-damage structure (track who didn't pay)
//! has no demonstrated primitive — both are GAP'd. The trigger is kept
//! as the end-step hook with an empty body.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Osseous Sticktwister");
    let scarecrow = reg.interner_mut().intern("Scarecrow");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(scarecrow);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            // GAP: Delirium intervening-if (four+ card types among graveyard cards) has no predicate.
            intervening_if: None,
            effect: delirium_effect,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn delirium_effect(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "each opponent may sacrifice a nonland permanent or discard a card; then deal damage
    // equal to power to each opponent who didn't" — no choose-then-track-who-declined primitive.
    Vec::new()
}
