//! Moonsilver Key — `{2}` artifact.
//! "{1}, {T}, Sacrifice this artifact: Search your library for an artifact
//! card with a mana ability or a basic land card, reveal it, put it into
//! your hand, then shuffle." Modeled as a tutor-to-hand over artifact or
//! land cards; the "with a mana ability" and "basic" refinements are not
//! expressible (see GAP below).

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
    let name = reg.interner_mut().intern("Moonsilver Key");
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
                text: "{1}, {T}, Sacrifice this artifact: Search your \
                       library for an artifact card with a mana ability or \
                       a basic land card, reveal it, put it into your hand, \
                       then shuffle."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
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
                effect: tutor_rock_or_land,
            },
        ),
    )
}

fn tutor_rock_or_land(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "an artifact card with a mana ability or a basic land card" —
    // the per-branch refinements ("with a mana ability" on the artifact
    // half, "basic" on the land half) are not expressible in ObjectFilter;
    // this tutors any artifact or land card.
    vec![Effect::TutorToHand {
        player: ctx.controller,
        filter: ObjectFilter::new().with_types_any(TypeLine(
            TypeLine::ARTIFACT | TypeLine::LAND,
        )),
        reveal: true,
    }]
}
