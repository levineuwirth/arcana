//! Sultai Devotee — `{1}{G}` 2/1 Creature — Zombie Snake Druid. Green.
//!
//! Oracle text:
//! * Deathtouch
//! * {1}: Add {B}, {G}, or {U}. Activate only once each turn.
//!
//! Decomposition:
//! * Keyword line → `keywords: vec![KeywordAbility::Deathtouch]`.
//! * "{1}: Add {B}, {G}, or {U}." is one ability with a color choice; the
//!   demonstrated path models a color choice as one mana ability per color.
//!   Each carries `once_per_turn: true` for "Activate only once each turn".
//!   (Minor fidelity gap: once-per-turn is tracked per ability index, so the
//!   three color modes are independently gated rather than sharing one
//!   per-turn limit — the closest expression with the demonstrated API.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sultai Devotee");
    let zombie = reg.interner_mut().intern("Zombie");
    let snake = reg.interner_mut().intern("Snake");
    let druid = reg.interner_mut().intern("Druid");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(snake);
    subtypes.0.insert(druid);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Deathtouch],
        ..Default::default()
    };

    let one_once = || ActivationCost {
        mana_cost: ManaCost::parse("{1}").expect("valid cost"),
        once_per_turn: true,
        ..ActivationCost::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Add {B}. Activate only once each turn.".into(),
                cost: one_once(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_black,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Add {G}. Activate only once each turn.".into(),
                cost: one_once(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Add {U}. Activate only once each turn.".into(),
                cost: one_once(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_blue,
            }),
    )
}

fn add_black(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}

fn add_green(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn add_blue(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}
