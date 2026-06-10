//! Boseiju, Who Endures — Legendary Land.
//! "{T}: Add {G}." and "Channel — {1}{G}, Discard this card: Destroy
//! target artifact, enchantment, or nonbasic land an opponent controls.
//! That player may search their library for a land card with a basic
//! land type, put it onto the battlefield, then shuffle. This ability
//! costs {1} less to activate for each legendary creature you control."
//!
//! Channel is a hand-activated ability: `discard_self: true` +
//! `ActivationZone::Hand`.
//! GAP: the "costs {1} less for each legendary creature you control"
//! cost reduction is not expressible; the printed {1}{G} is charged.
//! GAP: "may search" is modeled as a mandatory search.

use arcana_core::effects::Effect;
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
    CardId, ColorSet, ManaColor, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Boseiju, Who Endures");
    // Pre-intern the basic land types so the resolver's lookups resolve.
    let _plains = reg.interner_mut().intern("Plains");
    let _island = reg.interner_mut().intern("Island");
    let _swamp = reg.interner_mut().intern("Swamp");
    let _mountain = reg.interner_mut().intern("Mountain");
    let _forest = reg.interner_mut().intern("Forest");
    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::new(),
        types: TypeLine::LAND.into(),
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Add {G}.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: Vec::new(),
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_green_mana,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Channel — {1}{G}, Discard this card: Destroy target artifact, enchantment, or nonbasic land an opponent controls. That player may search their library for a land card with a basic land type, put it onto the battlefield, then shuffle.".into(),
                // GAP: "costs {1} less to activate for each legendary creature
                // you control" — dynamic activation-cost reduction is not
                // expressible; the printed {1}{G} is always charged.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                    discard_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::new()
                            .with_types_any(TypeLine(
                                TypeLine::ARTIFACT
                                    | TypeLine::ENCHANTMENT
                                    | TypeLine::LAND,
                            ))
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
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: channel_destroy,
            }),
    )
}

fn add_green_mana(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Green, ctx.source)],
    }]
}

fn channel_destroy(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let owner = script::target_controller(state, *id, ctx.controller);
    let mut basics = Vec::new();
    for n in ["Plains", "Island", "Swamp", "Mountain", "Forest"] {
        if let Some(s) = reg.interner().lookup(n) {
            basics.push(s);
        }
    }
    vec![
        Effect::DestroyPermanent { target: *id },
        // GAP: oracle says that player MAY search — modeled as mandatory.
        Effect::TutorToBattlefield {
            player: owner,
            filter: ObjectFilter::new()
                .with_types(TypeLine::LAND.into())
                .with_subtypes_any(basics),
            tapped: false,
        },
    ]
}
