//! Dust Stalker — `{2}{B}{R}` 5/3 Eldrazi with Haste (Devoid — has no
//! color).
//! "At the beginning of each end step, if you control no other
//! colorless creatures, return this creature to its owner's hand."
//!
//! Devoid makes the card colorless despite its `{B}{R}` cost, so colors
//! are `ColorSet::colorless()` (Devoid is not a `KeywordAbility` variant
//! — it is captured purely via the color set). Haste is a base keyword.
//! The end-step trigger is intervening-if gated: it bounces this
//! creature when you control no other colorless creatures (modeled as
//! "at most one colorless creature" — this creature itself — since
//! self-exclusion in the count isn't directly available).

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

fn all_colors() -> ColorSet {
    ColorSet::white()
        | ColorSet::blue()
        | ColorSet::black()
        | ColorSet::red()
        | ColorSet::green()
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dust Stalker");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::End,
                    whose: ControllerConstraint::Any,
                },
                intervening_if: Some(if_no_other_colorless),
                effect: bounce_self,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_no_other_colorless(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    // "no other colorless creatures" — approximate as at-most-one colorless
    // creature you control (this creature itself).
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .without_colors(all_colors());
    conditions::you_control_at_most(s, you, &filter, 1)
}

fn bounce_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ReturnToHand { target: trig.source }]
}
