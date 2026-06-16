//! Nissa, Vital Force — `{3}{G}{G}` Legendary Planeswalker — Nissa.
//! Starting loyalty 5 (oracle).
//!
//! +1: Untap target land you control. Until your next turn, it becomes a 5/5
//!     Elemental creature with haste. It's still a land.
//!     PARTIAL: the untap is modeled. GAP: animate-the-land-as-a-5/5-
//!     Elemental-with-haste-still-a-land (a typed/keyworded animation until
//!     your next turn) is not expressible as a single loyalty effect.
//! −3: Return target permanent card from your graveyard to your hand.
//!     GAP: graveyard-targeting loyalty ability (no any-graveyard target
//!     sentinel in the demonstrated surface).
//! −6: You get an emblem with "Whenever a land you control enters, you may
//!     draw a card." GAP: emblem creation not modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nissa, Vital Force");
    let nissa = reg.interner_mut().intern("Nissa");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nissa);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Untap target land you control. Until your next \
                       turn, it becomes a 5/5 Elemental creature with haste. \
                       It's still a land.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::permanent()
                            .with_types(TypeLine::LAND.into())
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_untap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Return target permanent card from your graveyard to \
                       your hand.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: You get an emblem.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_six_emblem,
            }),
    )
}

fn plus_one_untap(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // PARTIAL: untap the land. GAP: the 5/5 Elemental animation until your
    //          next turn.
    vec![Effect::Untap { target: *id }]
}

fn minus_three_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: graveyard-targeting return (no any-graveyard target sentinel).
    Vec::new()
}

fn minus_six_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem creation not modeled.
    Vec::new()
}
