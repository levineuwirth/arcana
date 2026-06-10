//! Adarkar Wastes — Land (painland).
//! "{T}: Add {C}." and "{T}: Add {W} or {U}. This land deals 1 damage
//! to you."
//!
//! The colored abilities each add their pip and deal 1 damage to the
//! controller. Because their effect is not purely `AddMana`, they are
//! wired with `is_mana_ability: false` per the engine convention
//! (CR-wise they are mana abilities; the damage rider forces the
//! non-mana-ability path here).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Adarkar Wastes");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {W}. This land deals 1 damage to you.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_white_pain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {U}. This land deals 1 damage to you.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_blue_pain,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn add_white_pain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::White, ctx.source)],
        },
        Effect::DealDamage {
            target: DamageTarget::Player(ctx.controller),
            amount: 1,
            source: ctx.source,
        },
    ]
}

fn add_blue_pain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
        },
        Effect::DealDamage {
            target: DamageTarget::Player(ctx.controller),
            amount: 1,
            source: ctx.source,
        },
    ]
}
