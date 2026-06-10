//! Shadowblood Egg — `{1}` artifact.
//! "{2}, {T}, Sacrifice this artifact: Add {B}{R}. Draw a card."
//!
//! One activated ability whose cost combines mana + tap + sacrifice;
//! the effect both adds mana and draws, so it is NOT a mana ability
//! (CR 605 — a mana ability's only result must be adding mana).

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
    let name = reg.interner_mut().intern("Shadowblood Egg");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{2}, {T}, Sacrifice this artifact: Add {B}{R}. Draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_mana_and_draw,
            },
        ),
    )
}

fn add_mana_and_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddMana {
            player: ctx.controller,
            mana: vec![
                ManaUnit::plain(ManaColor::Black, ctx.source),
                ManaUnit::plain(ManaColor::Red, ctx.source),
            ],
        },
        Effect::DrawCards { player: ctx.controller, count: 1 },
    ]
}
