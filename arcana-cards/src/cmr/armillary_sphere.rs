//! Armillary Sphere — `{2}` artifact.
//! "{2}, {T}, Sacrifice this artifact: Search your library for up to
//! two basic land cards, reveal them, put them into your hand, then
//! shuffle."
//!
//! Modeled as two successive single-card basic-land tutors to hand
//! (the engine shuffles automatically; "up to two" is approximated as
//! two searches, each of which may simply find nothing).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Armillary Sphere");
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
                text: "{2}, {T}, Sacrifice this artifact: Search your library for up to two basic land cards, reveal them, put them into your hand, then shuffle.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
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
                effect: fetch_two_basics,
            },
        ),
    )
}

fn fetch_two_basics(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::TutorToHand {
            player: ctx.controller,
            filter: basic_land_filter(),
            reveal: true,
        },
        Effect::TutorToHand {
            player: ctx.controller,
            filter: basic_land_filter(),
            reveal: true,
        },
    ]
}

fn basic_land_filter() -> ObjectFilter {
    ObjectFilter::new()
        .with_types(TypeLine::LAND.into())
        .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC))
}
