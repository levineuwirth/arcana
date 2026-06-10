//! Mana Screw — `{1}` artifact (Limited Edition Alpha, 1993).
//! "{1}: Flip a coin. If you win the flip, add {C}{C}. Activate only as
//! an instant." Coin-flip-gated mana: `Effect::FlipCoin` wrapping the
//! AddMana. Because the result is not unconditionally AddMana the
//! ability is wired as a stack-using activated ability
//! (`is_mana_ability: false`) with `is_instant_speed: true` — a
//! documented fidelity note (CR 605 would class it as a mana ability).

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
    let name = reg.interner_mut().intern("Mana Screw");
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
                text: "{1}: Flip a coin. If you win the flip, add {C}{C}. \
                       Activate only as an instant."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: true,
                face_gate: None,
                effect: flip_for_mana,
            },
        ),
    )
}

fn flip_for_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::FlipCoin {
        player: ctx.controller,
        win: Box::new(Effect::AddMana {
            player: ctx.controller,
            mana: vec![
                ManaUnit::plain(ManaColor::Colorless, ctx.source),
                ManaUnit::plain(ManaColor::Colorless, ctx.source),
            ],
        }),
        lose: None,
    }]
}
