//! Memory Vessel — `{3}{R}{R}` artifact.
//! "{T}, Exile this artifact: Each player exiles the top seven cards
//! of their library. Until your next turn, players may play cards they
//! exiled this way, and they can't play cards from their hand.
//! Activate only as a sorcery."
//!
//! Best-effort: each player gets `Effect::ImpulseExile` for seven
//! cards. GAP: the play permission lasts until end of turn (engine
//! limit) rather than "until your next turn", and the "can't play
//! cards from their hand" lock is not expressible.

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
    let name = reg.interner_mut().intern("Memory Vessel");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{T}, Exile this artifact: Each player exiles the top \
                       seven cards of their library. Until your next turn, \
                       players may play cards they exiled this way, and \
                       they can't play cards from their hand. Activate \
                       only as a sorcery."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    exile_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exile_seven_each,
            },
        ),
    )
}

/// "Each player exiles the top seven cards of their library. Until
/// your next turn, players may play cards they exiled this way…"
fn exile_seven_each(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ImpulseExile's play permission lapses at end of turn, not
    // "until your next turn", and the companion "can't play cards from
    // their hand" restriction is not expressible.
    let _ = ctx;
    let effects = script::all_players(state)
        .into_iter()
        .map(|p| Effect::ImpulseExile { player: p, count: 7 })
        .collect();
    vec![Effect::Sequence(effects)]
}
