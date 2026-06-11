//! Forum Filibuster — `{3}{W}{W}` enchantment.
//! "At the beginning of your upkeep, create a 2/1 white and black
//! Inkling creature token with flying. When you do, return up to one
//! target Aura or Equipment card from your graveyard to the battlefield
//! attached to that token."
//!
//! The upkeep token mint is faithful; the reflexive "when you do,
//! return … attached to that token" rider is a documented GAP (no
//! reflexive trigger, no attach-on-return primitive).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forum Filibuster");
    let _inkling = reg.interner_mut().intern("Inkling");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: mint_inkling,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…create a 2/1 white and black Inkling creature token with flying."
fn mint_inkling(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the reflexive rider — "When you do, return up to one target
    // Aura or Equipment card from your graveyard to the battlefield
    // attached to that token" — needs a reflexive trigger plus an
    // attach-on-return primitive; neither exists. Only the token is made.
    let inkling = reg.interner().lookup("Inkling").unwrap_or_default();
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(inkling);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: inkling,
            colors: ColorSet::white() | ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![KeywordAbility::Flying],
            abilities: vec![],
        },
    }]
}
