//! Mox Poison — `{0}` artifact (playtest card, Mystery Booster).
//! "{T}: Add one mana of any color. You get two poison counters."
//! The any-color choice is five mana abilities, one per WUBRG color.
//! Each adds its mana then gives the controller two poison counters via
//! Effect::GivePlayerCounters { kind: Poison, count: 2 }.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mox Poison");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(mana_ability("{T}: Add {W}.", add_white))
            .with_activated_ability(mana_ability("{T}: Add {U}.", add_blue))
            .with_activated_ability(mana_ability("{T}: Add {B}.", add_black))
            .with_activated_ability(mana_ability("{T}: Add {R}.", add_red))
            .with_activated_ability(mana_ability("{T}: Add {G}.", add_green)),
    )
}

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

fn add_of(color: ManaColor, ctx: &ActivationContext) -> Vec<Effect> {
    // "Add one mana of any color. You get two poison counters."
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(color, ctx.source)],
        },
        Effect::GivePlayerCounters { player: ctx.controller, kind: CounterKind::Poison, count: 2 },
    ]
}

fn add_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_of(ManaColor::White, ctx)
}

fn add_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_of(ManaColor::Blue, ctx)
}

fn add_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_of(ManaColor::Black, ctx)
}

fn add_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_of(ManaColor::Red, ctx)
}

fn add_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_of(ManaColor::Green, ctx)
}
