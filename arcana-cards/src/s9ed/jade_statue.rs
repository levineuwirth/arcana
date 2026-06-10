//! Jade Statue — `{4}` artifact (Limited Edition Alpha, 1993).
//! "{2}: This artifact becomes a 3/6 Golem artifact creature until
//! end of combat. Activate only during combat."
//! One animation activation.
//! // GAP: "until end of combat" has no Duration variant — modeled
//! with Duration::EndOfTurn.
//! // GAP: "Activate only during combat" timing restriction is not
//! expressible with the demonstrated API.
//! // GAP: the granted Golem subtype is not expressible (no
//! subtype-granting Effect).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jade Statue");
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
                text: "{2}: This artifact becomes a 3/6 Golem artifact creature until end of combat. Activate only during combat.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: animate,
            },
        ),
    )
}

fn animate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "until end of combat" approximated as Duration::EndOfTurn;
    // Golem subtype grant and the combat-only activation window are
    // not expressible.
    vec![
        Effect::AddType {
            target: ctx.source,
            types: TypeLine::CREATURE.into(),
            duration: Duration::EndOfTurn,
        },
        Effect::SetBasePT {
            target: ctx.source,
            power: 3,
            toughness: 6,
            duration: Duration::EndOfTurn,
        },
    ]
}
