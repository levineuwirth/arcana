//! Barbed Sextant — `{1}` artifact.
//! "{1}, {T}, Sacrifice this artifact: Add one mana of any color.
//! Draw a card at the beginning of the next turn's upkeep." The
//! any-color choice is modeled as five separately activatable mana
//! abilities, one per WUBRG color. The delayed draw is wired via
//! Effect::DelayedAction (DelayedWhen::NextUpkeep +
//! DelayedAction::ControllerDrawsCard) on each ability.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

fn sac_cost() -> ActivationCost {
    ActivationCost {
        mana_cost: ManaCost::parse("{1}").expect("valid cost"),
        tap: true,
        sacrifice: true,
        ..ActivationCost::default()
    }
}

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barbed Sextant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}, Sacrifice this artifact: Add {W}.".into(),
                cost: sac_cost(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_white,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}, Sacrifice this artifact: Add {U}.".into(),
                cost: sac_cost(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_blue,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}, Sacrifice this artifact: Add {B}.".into(),
                cost: sac_cost(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_black,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}, Sacrifice this artifact: Add {R}.".into(),
                cost: sac_cost(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_red,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}, Sacrifice this artifact: Add {G}.".into(),
                cost: sac_cost(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green,
            }),
    )
}

// Each ability: add one mana of its color and schedule "Draw a card at
// the beginning of the next turn's upkeep".
fn add_one_of(color: ManaColor, ctx: &ActivationContext) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![ManaUnit::plain(color, ctx.source)],
        },
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextUpkeep,
            action: DelayedAction::ControllerDrawsCard,
        },
    ]
}

fn add_white(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one_of(ManaColor::White, ctx)
}

fn add_blue(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one_of(ManaColor::Blue, ctx)
}

fn add_black(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one_of(ManaColor::Black, ctx)
}

fn add_red(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one_of(ManaColor::Red, ctx)
}

fn add_green(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    add_one_of(ManaColor::Green, ctx)
}
