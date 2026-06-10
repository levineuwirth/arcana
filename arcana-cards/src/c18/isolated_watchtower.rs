//! Isolated Watchtower — nonbasic land.
//! "{T}: Add {C}." and "{2}, {T}: Scry 1, then you may reveal the top
//! card of your library. If a basic land card is revealed this way,
//! put it onto the battlefield tapped. Activate only if an opponent
//! controls at least two more lands than you."
//!
//! GAP: the reveal-top/put-basic-onto-battlefield rider and the
//! "Activate only if an opponent controls at least two more lands
//! than you" gate are not expressible — only the Scry 1 is wired.

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
    let name = reg.interner_mut().intern("Isolated Watchtower");
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
                text: "{2}, {T}: Scry 1, then you may reveal the top card \
                       of your library. If a basic land card is revealed \
                       this way, put it onto the battlefield tapped. \
                       Activate only if an opponent controls at least two \
                       more lands than you."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: scry_and_reveal,
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

fn scry_and_reveal(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may reveal the top card of your library. If a basic land
    // card is revealed this way, put it onto the battlefield tapped" — no
    // reveal-top-in-place effect (RevealUntil would move a non-matching top
    // card). GAP: "Activate only if an opponent controls at least two more
    // lands than you" — activation precondition not expressible.
    vec![Effect::Scry { player: ctx.controller, count: 1 }]
}
