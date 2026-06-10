//! Rejuvenating Springs — nonbasic land.
//! "This land enters tapped unless you have two or more opponents."
//! and "{T}: Add {G} or {U}."
//! Modeled as enters-tapped plus two mana abilities.
//! GAP fidelity: the enters-tapped condition ('unless you have two or
//! more opponents') is not expressible — EntersWithSpec::Tapped is
//! unconditional. In the engine's two-player games you always have
//! exactly one opponent, so unconditional tapped is behaviorally
//! correct there.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaUnit;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rejuvenating Springs");
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
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green_mana,
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
                effect: add_blue_mana,
            }),
    )
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn add_blue_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Blue, ctx.source)],
    }]
}
