//! A-Navigation Orb — `{3}` artifact (Alchemy rebalance).
//! "{1}, {T}, Sacrifice Navigation Orb: Search your library for up to
//! two basic land cards and/or Gate cards, reveal those cards, put one
//! onto the battlefield tapped and the other into your hand, then
//! shuffle."
//!
//! Approximated as two chained tutors — one basic land to the
//! battlefield tapped and one basic land to hand. The "and/or Gate
//! cards" option and the up-to-two split choice are GAP'd (a single
//! `ObjectFilter` cannot express basic-supertype OR Gate-subtype, and
//! the tutors are mandatory single-card searches).

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
    let name = reg.interner_mut().intern("A-Navigation Orb");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}, {T}, Sacrifice Navigation Orb: Search your library for up to two basic land cards and/or Gate cards, reveal those cards, put one onto the battlefield tapped and the other into your hand, then shuffle.".into(),
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
            effect: search_two_lands,
        }),
    )
}

fn search_two_lands(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "basic land cards and/or Gate cards" — an ObjectFilter cannot
    // express (BASIC supertype OR Gate subtype); only the basic-land half is
    // modeled. The "up to two" optionality is also approximated as two
    // mandatory searches (battlefield tapped + hand).
    vec![
        Effect::TutorToBattlefield {
            player: ctx.controller,
            filter: ObjectFilter::new()
                .with_types(TypeLine::LAND.into())
                .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
            tapped: true,
        },
        Effect::TutorToHand {
            player: ctx.controller,
            filter: ObjectFilter::new()
                .with_types(TypeLine::LAND.into())
                .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
            reveal: true,
        },
    ]
}
