//! Wrenn and Realmbreaker — `{1}{G}{G}` Legendary Planeswalker — Wrenn,
//! starting loyalty 5.
//!
//! Static: "Lands you control have '{T}: Add one mana of any color.'" —
//! a continuous static ability, not a loyalty ability; not modeled here.
//!
//! * `+1`: Up to one target land you control becomes a 3/3 Elemental
//!   creature with vigilance, hexproof, and haste until your next turn.
//!   GAP'd: the "until your next turn" duration is not in the demonstrated
//!   `Duration` surface (only `EndOfTurn` / `WhileSourceOnBattlefield`).
//! * `−2`: Mill three cards (expressible); the "put a permanent card from
//!   among the milled cards into your hand" rider is not expressible.
//! * `−7`: Emblem — GAP'd.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrenn and Realmbreaker");
    let wrenn = reg.interner_mut().intern("Wrenn");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wrenn);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}").expect("valid cost")),
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
                text: "+1: Up to one target land you control becomes a 3/3 \
                       Elemental creature with vigilance, hexproof, and haste \
                       until your next turn. It's still a land.".into(),
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
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_animate,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−2: Mill three cards. You may put a permanent card from \
                       among the milled cards into your hand.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_mill,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You get an emblem with \"You may play lands and cast \
                       permanent spells from your graveyard.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_emblem,
            }),
    )
}

fn plus_one_animate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "until your next turn" duration is not in the demonstrated
    // Duration surface; the animate-land bundle (SetBasePT + AddType +
    // keyword grants) requires that duration to be faithful.
    Vec::new()
}

fn minus_two_mill(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "put a permanent card from among the milled cards into your
    // hand" rider is not expressible; the mill is.
    vec![Effect::Mill { player: ctx.controller, count: 3 }]
}

fn minus_seven_emblem(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: emblem granting graveyard-casting permission.
    Vec::new()
}
