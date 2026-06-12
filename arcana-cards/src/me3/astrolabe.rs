//! Astrolabe — `{3}` artifact (Visions).
//! "{1}, {T}, Sacrifice this artifact: Add two mana of any one color.
//! Draw a card at the beginning of the next turn's upkeep." The
//! any-one-color choice is modeled as five activated abilities, one
//! per WUBRG color, each adding two mana of that color. The delayed
//! draw is wired via Effect::DelayedAction (DelayedWhen::NextUpkeep +
//! DelayedAction::ControllerDrawsCard).

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Astrolabe");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(sac_ability(
                "{1}, {T}, Sacrifice this artifact: Add {W}{W}.",
                add_two_white,
            ))
            .with_activated_ability(sac_ability(
                "{1}, {T}, Sacrifice this artifact: Add {U}{U}.",
                add_two_blue,
            ))
            .with_activated_ability(sac_ability(
                "{1}, {T}, Sacrifice this artifact: Add {B}{B}.",
                add_two_black,
            ))
            .with_activated_ability(sac_ability(
                "{1}, {T}, Sacrifice this artifact: Add {R}{R}.",
                add_two_red,
            ))
            .with_activated_ability(sac_ability(
                "{1}, {T}, Sacrifice this artifact: Add {G}{G}.",
                add_two_green,
            )),
    )
}

fn sac_ability(
    text: &str,
    effect: fn(&GameState, &ActivationContext, &CardRegistry) -> Vec<Effect>,
) -> ActivatedAbilityDef {
    ActivatedAbilityDef {
        text: text.into(),
        cost: ActivationCost {
            mana_cost: ManaCost::parse("{1}").expect("valid cost"),
            tap: true,
            sacrifice: true,
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

fn add_two_of(color: ManaColor, ctx: &ActivationContext) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![
                ManaUnit::plain(color, ctx.source),
                ManaUnit::plain(color, ctx.source),
            ],
        },
        // "Draw a card at the beginning of the next turn's upkeep."
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextUpkeep,
            action: DelayedAction::ControllerDrawsCard,
        },
    ]
}

fn add_two_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_two_of(ManaColor::White, ctx)
}

fn add_two_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_two_of(ManaColor::Blue, ctx)
}

fn add_two_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_two_of(ManaColor::Black, ctx)
}

fn add_two_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_two_of(ManaColor::Red, ctx)
}

fn add_two_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_two_of(ManaColor::Green, ctx)
}
