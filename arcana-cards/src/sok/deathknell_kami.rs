//! Deathknell Kami — `{1}{B}` 0/1 Spirit with Flying and Soulshift 1.
//! "{2}: This creature gets +1/+1 until end of turn. Sacrifice it at the
//! beginning of the next end step.
//! Soulshift 1 (When this creature dies, you may return target Spirit card with
//! mana value 1 or less from your graveyard to your hand.)"
//!
//! Flying + Soulshift(1) are base keywords (the engine synthesizes the
//! Soulshift dies-trigger). The activated ability pumps the source and
//! schedules a self-sacrifice at the next end step.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::effects::{DelayedAction, DelayedWhen};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deathknell Kami");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Soulshift(1)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: This creature gets +1/+1 until end of turn. \
                       Sacrifice it at the beginning of the next end step."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_and_schedule_sacrifice,
            }),
    )
}

fn pump_and_schedule_sacrifice(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Pump {
            target: ctx.source,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        },
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        },
    ]
}
