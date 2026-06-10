//! Throne of Geth — `{2}` artifact (Scars of Mirrodin).
//! "{T}, Sacrifice an artifact: Proliferate."
//!
//! The sacrificed artifact is a CHOSEN permanent (`sacrifice_other`);
//! the engine's enumeration always excludes this card itself — a
//! documented approximation for "Sacrifice an artifact" (which could
//! legally be this card).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Throne of Geth");
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
                text: "{T}, Sacrifice an artifact: Proliferate.".into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice_other: Some(ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: proliferate,
            },
        ),
    )
}

fn proliferate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Proliferate]
}
