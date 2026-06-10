//! Ancient Ziggurat — nonbasic land.
//! "{T}: Add one mana of any color. Spend this mana only to cast a creature
//! spell." Modeled as FIVE mana abilities, one per WUBRG color; the
//! creature-spells-only spend restriction is a GAP.

use arcana_core::effects::Effect;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::mana::ManaUnit;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

fn mana_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost::tap_only(),
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ancient Ziggurat");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        ..Default::default()
    };
    // GAP: "Spend this mana only to cast a creature spell" — mana spend
    // restrictions are not expressible; the plain any-color mana abilities
    // are emitted without the restriction.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(mana_ability("{T}: Add {W}.", add_white))
            .with_activated_ability(mana_ability("{T}: Add {U}.", add_blue))
            .with_activated_ability(mana_ability("{T}: Add {B}.", add_black))
            .with_activated_ability(mana_ability("{T}: Add {R}.", add_red))
            .with_activated_ability(mana_ability("{T}: Add {G}.", add_green)),
    )
}

fn add_one(ctx: &ActivationContext, color: ManaColor) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(color, ctx.source)],
    }]
}

fn add_white(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::White)
}

fn add_blue(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Blue)
}

fn add_black(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Black)
}

fn add_red(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Red)
}

fn add_green(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    add_one(ctx, ManaColor::Green)
}
