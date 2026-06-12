//! Spectral Searchlight — `{3}` artifact (Ravnica: City of Guilds, 2005).
//! "{T}: Choose a player. That player adds one mana of any color they
//! choose." Modeled as five mana abilities (one per WUBRG color); each
//! wraps its AddMana in Effect::ChoosePlayerThen so the activator picks
//! the player who receives the mana. GAP: the COLOR is chosen by the
//! activator (via which of the five abilities is activated), not by the
//! chosen player as the oracle text says.

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
    let name = reg.interner_mut().intern("Spectral Searchlight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
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

// "Choose a player. That player adds one mana of [this ability's
// color]" — Effect::ChoosePlayerThen posts the player pick and
// substitutes the chosen player into the AddMana (the placeholder
// player 0 below is overwritten). GAP: the chosen player's own color
// choice is approximated by the activator's choice of ability.
fn choose_player_add(color: ManaColor, ctx: &ActivationContext) -> Vec<Effect> {
    vec![Effect::ChoosePlayerThen {
        chooser: ctx.controller,
        opponents_only: false,
        then: Box::new(Effect::AddMana {
            player: 0,
            mana: vec![ManaUnit::plain(color, ctx.source)],
        }),
    }]
}

fn add_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    choose_player_add(ManaColor::White, ctx)
}

fn add_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    choose_player_add(ManaColor::Blue, ctx)
}

fn add_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    choose_player_add(ManaColor::Black, ctx)
}

fn add_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    choose_player_add(ManaColor::Red, ctx)
}

fn add_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    choose_player_add(ManaColor::Green, ctx)
}
