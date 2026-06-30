//! Celestial Colonnade — Worldwake-style creature-land. "This land enters tapped." Taps for
//! its colors, and "{3}{W}{U}: until end of turn this land becomes a 4/4
//! creature with flying, vigilance. It's still a land." Animation reuses the
//! add_type + set_pt (+ grant_keyword) layer idiom (cf. Crew).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};
use arcana_core::effects::KeywordAbility;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Celestial Colonnade");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Tapped)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {W}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_white,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {U}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_blue,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W}{U}: This land becomes a 4/4 creature until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W}{U}").expect("valid cost"),
                    ..Default::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: animate,
            }),
    )
}

fn animate(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![
        Effect::InstallContinuousEffect { effect: ContinuousEffect::add_type(ctx.source, ctx.source, TypeLine::CREATURE.into(), Duration::EndOfTurn) },
        Effect::InstallContinuousEffect { effect: ContinuousEffect::set_pt(ctx.source, ctx.source, 4, 4, Duration::EndOfTurn) },
        Effect::InstallContinuousEffect { effect: ContinuousEffect::grant_keyword(ctx.source, ctx.source, KeywordAbility::Flying, Duration::EndOfTurn) },
        Effect::InstallContinuousEffect { effect: ContinuousEffect::grant_keyword(ctx.source, ctx.source, KeywordAbility::Vigilance, Duration::EndOfTurn) },
    ]
}

fn add_white(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)] }]
}

fn add_blue(_s: &GameState, ctx: &ActivationContext, _r: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana { player: ctx.controller, mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)] }]
}
