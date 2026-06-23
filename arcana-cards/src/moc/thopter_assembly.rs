//! Thopter Assembly — `{6}` 5/5 Artifact Creature — Thopter with Flying.
//!
//! Oracle:
//! * Flying
//! * At the beginning of your upkeep, if you control no Thopters other than
//!   this creature, return this creature to its owner's hand and create
//!   five 1/1 colorless Thopter artifact creature tokens with flying.
//!
//! Flying is a base keyword. The upkeep trigger is gated by an
//! intervening-if (CR 603.4): "you control no Thopters other than this" =
//! you control at most one Thopter (counting itself). Effect bounces this
//! creature and mints five flying Thopter tokens.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thopter Assembly");
    let thopter = reg.interner_mut().intern("Thopter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thopter);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![KeywordAbility::Flying],
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
                intervening_if: Some(if_no_other_thopters),
                effect: bounce_and_make_thopters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// "You control no Thopters other than this creature": at most one Thopter
/// total under your control (it counts itself).
fn if_no_other_thopters(
    s: &GameState,
    _src: ObjectId,
    you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    let filter = script::subtype_filter(reg, "Thopter");
    conditions::you_control_at_most(s, you, &filter, 1)
}

fn bounce_and_make_thopters(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = vec![Effect::ReturnToHand { target: trig.source }];
    let thopter_name = reg
        .interner()
        .lookup("Thopter")
        .unwrap_or_default();
    let token = make_thopter_token(thopter_name);
    for _ in 0..5 {
        effects.push(Effect::CreateToken {
            controller: trig.controller,
            token: token.clone(),
        });
    }
    effects
}

fn make_thopter_token(thopter_name: arcana_core::types::SmallString) -> TokenDefinition {
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(thopter_name);
    TokenDefinition {
        name: thopter_name,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying],
        abilities: vec![],
    }
}
