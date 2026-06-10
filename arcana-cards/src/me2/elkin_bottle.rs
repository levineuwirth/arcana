//! Elkin Bottle — `{3}` artifact (Ice Age, 1995).
//! "{3}, {T}: Exile the top card of your library. Until the beginning
//! of your next upkeep, you may play that card."
//! Wired with [`Effect::ImpulseExile`] — the engine's exile-and-may-play
//! primitive. FIDELITY GAP: the engine's play-permission lapses at end
//! of turn rather than at the beginning of your next upkeep (the oracle
//! window is slightly wider).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elkin Bottle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{3}, {T}: Exile the top card of your library. Until \
                       the beginning of your next upkeep, you may play that \
                       card."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: impulse_one,
            },
        ),
    )
}

fn impulse_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: oracle window is "until the beginning of your next upkeep";
    // ImpulseExile's permission lapses at end of turn.
    vec![Effect::ImpulseExile { player: ctx.controller, count: 1 }]
}
