//! Trampled Lotus — `{2}` artifact (Unknown Event, un-set).
//! "{T}, Sacrifice Trampled Lotus: Tap up to three lands your
//! opponents control for mana. This ability has splitter second."
//!
//! GAP: 'for mana' (forcing the tapped lands to produce mana for
//! anyone) and 'splitter second' (un-set uncounterability) are not
//! modeled — only the tap of up to three opponent lands is wired.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Trampled Lotus");
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
                text: "{T}, Sacrifice Trampled Lotus: Tap up to three lands \
                       your opponents control for mana. This ability has \
                       splitter second."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::UpTo(3),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_lands,
            },
        ),
    )
}

fn tap_lands(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: 'for mana' — forcing the tapped lands to produce mana is not
    // modeled; GAP: 'splitter second' — un-set uncounterability.
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::Tap { target: *id }),
            _ => None,
        })
        .collect()
}
