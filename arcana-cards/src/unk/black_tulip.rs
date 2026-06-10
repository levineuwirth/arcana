//! Black Tulip — `{0}` artifact.
//! "{T}, Exile Black Tulip: Add three mana of any one color. You
//! can't activate this ability until you've begun your sixth turn of
//! the game." The any-one-color choice is modeled as five activated
//! abilities, one per WUBRG color, each paying tap + exile-self and
//! adding three mana of that color.
//!
//! GAP: "You can't activate this ability until you've begun your
//! sixth turn of the game" — turn-count activation gates are not
//! expressible; the abilities are activatable from turn one.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Black Tulip");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{0}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    // GAP: "You can't activate this ability until you've begun your
    // sixth turn of the game" — no turn-count activation gate.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(tulip_ability(
                "{T}, Exile Black Tulip: Add {W}{W}{W}.",
                add_three_white,
            ))
            .with_activated_ability(tulip_ability(
                "{T}, Exile Black Tulip: Add {U}{U}{U}.",
                add_three_blue,
            ))
            .with_activated_ability(tulip_ability(
                "{T}, Exile Black Tulip: Add {B}{B}{B}.",
                add_three_black,
            ))
            .with_activated_ability(tulip_ability(
                "{T}, Exile Black Tulip: Add {R}{R}{R}.",
                add_three_red,
            ))
            .with_activated_ability(tulip_ability(
                "{T}, Exile Black Tulip: Add {G}{G}{G}.",
                add_three_green,
            )),
    )
}

fn tulip_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            tap: true,
            exile_self: true,
            ..ActivationCost::default()
        },
        target_requirements: Vec::new(),
        is_mana_ability: true,
        is_loyalty_ability: false,
        activation_zone: ActivationZone::Battlefield,
        is_instant_speed: false,
        face_gate: None,
        effect,
    }
}

fn add_three_of(color: ManaColor, ctx: &ActivationContext) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![
            ManaUnit::plain(color, ctx.source),
            ManaUnit::plain(color, ctx.source),
            ManaUnit::plain(color, ctx.source),
        ],
    }]
}

fn add_three_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_three_of(ManaColor::White, ctx)
}

fn add_three_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_three_of(ManaColor::Blue, ctx)
}

fn add_three_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_three_of(ManaColor::Black, ctx)
}

fn add_three_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_three_of(ManaColor::Red, ctx)
}

fn add_three_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_three_of(ManaColor::Green, ctx)
}
