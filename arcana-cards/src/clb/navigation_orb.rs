//! Navigation Orb — {3} artifact (March of the Machine Commander,
//! 2023). "{2}, {T}, Sacrifice this artifact: Search your library for
//! up to two basic land cards and/or Gate cards, reveal those cards,
//! put one onto the battlefield tapped and the other into your hand,
//! then shuffle." Approximated as one basic-land tutor to the
//! battlefield tapped plus one basic-land tutor to hand.

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
    let name = reg.interner_mut().intern("Navigation Orb");
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
                text: "{2}, {T}, Sacrifice this artifact: Search your \
                       library for up to two basic land cards and/or Gate \
                       cards, reveal those cards, put one onto the \
                       battlefield tapped and the other into your hand, \
                       then shuffle."
                    .into(),
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
                effect: fetch_lands,
            },
        ),
    )
}

fn fetch_lands(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "basic land cards and/or Gate cards" — an OR of basic-land
    // and Gate-subtype filters is not expressible in one ObjectFilter;
    // only the basic-land half is searched. The "up to two ... put one
    // onto the battlefield and the other into your hand" choice is
    // flattened into one tutor to the battlefield (tapped) plus one
    // tutor to hand.
    vec![
        Effect::TutorToBattlefield {
            player: ctx.controller,
            filter: ObjectFilter::new()
                .with_types(TypeLine::LAND.into())
                .with_supertypes(
                    SupertypeSet::new().with(SupertypeSet::BASIC),
                ),
            tapped: true,
        },
        Effect::TutorToHand {
            player: ctx.controller,
            filter: ObjectFilter::new()
                .with_types(TypeLine::LAND.into())
                .with_supertypes(
                    SupertypeSet::new().with(SupertypeSet::BASIC),
                ),
            reveal: true,
        },
    ]
}
