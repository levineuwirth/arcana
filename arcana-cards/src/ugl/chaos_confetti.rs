//! Chaos Confetti — `{4}` artifact (Unglued, 1998).
//! "{4}, {T}: Tear this artifact into pieces. Throw the pieces onto the
//! battlefield from a distance of at least five feet. Destroy each
//! permanent that a piece touches. Remove the pieces from the game."
//!
//! A dexterity card: the physical tear-and-throw has no digital model. The
//! activation is wired at its printed cost; the only deterministic clause —
//! the card itself ends up removed from the game — is modeled by exiling
//! this artifact; the throw is a GAP.

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
    let name = reg.interner_mut().intern("Chaos Confetti");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{4}, {T}: Tear this artifact into pieces. Throw the \
                       pieces onto the battlefield; destroy each permanent \
                       that a piece touches. Remove the pieces from the \
                       game."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: confetti,
            },
        ),
    )
}

/// The throw is unmodelable; the card removing itself from the game is real.
fn confetti(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the dexterity mechanic ("throw the pieces ... destroy each
    // permanent that a piece touches") has no digital model. Only the card
    // leaving the game is emitted (exile self).
    vec![Effect::ExilePermanent { target: ctx.source }]
}
