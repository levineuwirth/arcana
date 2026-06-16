//! The Locust God — `{4}{U}{R}` 4/4 Legendary God.
//! Flying.
//! Whenever you draw a card, create a 1/1 blue and red Insect creature
//! token with flying and haste.
//! {2}{U}{R}: Draw a card, then discard a card.
//! When The Locust God dies, return it to its owner's hand at the
//! beginning of the next end step.

use arcana_core::effects::{
    DelayedAction, DelayedWhen, DiscardChoice, Effect, KeywordAbility, TokenDefinition,
};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Locust God");
    let god = reg.interner_mut().intern("God");
    let _insect = reg.interner_mut().intern("Insect");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(god);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{R}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CardDrawn {
                    player: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: make_insect,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_return,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}{R}: Draw a card, then discard a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: loot,
            }),
    )
}

fn make_insect(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let insect = reg.interner().lookup("Insect").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(insect);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: insect,
            colors: ColorSet::blue() | ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying, KeywordAbility::Haste],
            abilities: vec![],
        },
    }]
}

fn dies_return(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::DelayedAction {
        source: trig.source,
        controller: trig.controller,
        when: DelayedWhen::NextEndStep,
        action: DelayedAction::ReturnToHand,
    }]
}

fn loot(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::DrawCards { player: ctx.controller, count: 1 },
        Effect::Discard {
            player: ctx.controller,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        },
    ]
}
