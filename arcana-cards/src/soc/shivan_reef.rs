//! Shivan Reef — nonbasic land (Apocalypse, 2001).
//! "{T}: Add {C}." and "{T}: Add {U} or {R}. This land deals 1 damage
//! to you." The painland: the colorless ability is a pure mana
//! ability; the colored abilities also deal 1 damage to you, so per
//! the only-AddMana rule they are wired as non-mana activations.

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
    let name = reg.interner_mut().intern("Shivan Reef");
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
                text: "{T}: Add {U}. This land deals 1 damage to you.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                // Damage rider means the effect is not AddMana-only, so
                // this cannot be flagged as a mana ability (fidelity note:
                // it therefore uses the stack).
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: add_blue_mana_pain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {R}. This land deals 1 damage to you.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: add_red_mana_pain,
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

fn add_blue_mana_pain(
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

fn add_red_mana_pain(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(ManaColor::Red, ctx.source)],
        },
        Effect::DealDamage {
            target: DamageTarget::Player(ctx.controller),
            amount: 1,
            source: ctx.source,
        },
    ]
}
