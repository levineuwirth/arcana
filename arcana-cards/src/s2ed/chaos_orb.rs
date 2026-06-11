//! Chaos Orb — `{2}` artifact (Alpha, 1993).
//! "{1}, {T}: If this artifact is on the battlefield, flip it onto the
//! battlefield from a height of at least one foot. If this artifact turns
//! over completely at least once during the flip, destroy all nontoken
//! permanents it touches. Then destroy this artifact."
//!
//! A dexterity card: the physical flip has no digital model. The activation
//! is wired at its printed cost; the only deterministic clause — "Then
//! destroy this artifact." — resolves; the flip itself is a GAP.

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
    let name = reg.interner_mut().intern("Chaos Orb");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{1}, {T}: Flip this artifact onto the battlefield; \
                       destroy all nontoken permanents it touches. Then \
                       destroy this artifact."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: flip_and_self_destruct,
            },
        ),
    )
}

/// The flip is unmodelable; the trailing self-destruction is real.
fn flip_and_self_destruct(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the physical coin-flip/placement ("flip it onto the battlefield
    // ... destroy all nontoken permanents it touches") is a dexterity
    // mechanic with no digital model. Only "Then destroy this artifact."
    // is emitted.
    vec![Effect::DestroyPermanent { target: ctx.source }]
}
