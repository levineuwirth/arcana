//! Tainted Sigil — `{1}{W}{B}` artifact (Shadowmoor, 2008).
//! "{T}, Sacrifice this artifact: You gain life equal to the total life
//! lost by all players this turn."
//!
//! A two-color (W/B) non-creature artifact with one sacrifice
//! activation. The amount is dynamic — sum of life lost this turn across
//! all players — computed via script::life_lost_this_turn over
//! script::all_players, then fed into a literal-amount Effect::GainLife.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tainted Sigil");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Sacrifice this artifact: You gain life equal to the \
                   total life lost by all players this turn."
                .into(),
            cost: ActivationCost {
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
            effect: gain_total_life_lost,
        }),
    )
}

fn gain_total_life_lost(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let total: u32 = script::all_players(state)
        .into_iter()
        .map(|p| script::life_lost_this_turn(state, p))
        .sum();
    vec![Effect::GainLife {
        player: ctx.controller,
        amount: total,
    }]
}
