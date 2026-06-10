//! Volatile Fault — nonbasic land, subtype Cave.
//! "{T}: Add {C}." and "{1}, {T}, Sacrifice this land: Destroy target
//! nonbasic land an opponent controls. That player may search their
//! library for a basic land card, put it onto the battlefield, then
//! shuffle. You create a Treasure token."
//!
//! GAP: "That player MAY search" — `Effect::TutorToBattlefield` is
//! mandatory; the opt-out is not expressible.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, ManaColor, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Volatile Fault");
    let cave = reg.interner_mut().intern("Cave");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cave);
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {C}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_colorless_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}, {T}, Sacrifice this land: Destroy target nonbasic land an opponent controls. That player may search their library for a basic land card, put it onto the battlefield, then shuffle. You create a Treasure token.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types(TypeLine::LAND.into())
                            .without_supertypes(
                                SupertypeSet::new().with(SupertypeSet::BASIC),
                            )
                            .controlled_by(ControllerConstraint::Opponent),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: destroy_and_ramp,
            }),
    )
}

fn add_colorless_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Colorless, ctx.source)],
    }]
}

fn destroy_and_ramp(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let owner = script::target_controller(state, *id, ctx.controller);
    vec![
        Effect::DestroyPermanent { target: *id },
        // GAP: oracle says that player MAY search — modeled as mandatory.
        Effect::TutorToBattlefield {
            player: owner,
            filter: ObjectFilter::new()
                .with_types(TypeLine::LAND.into())
                .with_supertypes(SupertypeSet::new().with(SupertypeSet::BASIC)),
            tapped: false,
        },
        Effect::CreateCommodityToken {
            controller: ctx.controller,
            kind: CommodityToken::Treasure,
            count: 1,
        },
    ]
}
