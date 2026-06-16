//! Elspeth, Undaunted Hero — `{2}{W}{W}{W}` Legendary Planeswalker — Elspeth.
//! Starting loyalty 5.
//!
//! +2: Put a +1/+1 counter on each of up to two target creatures.
//! −2: Search your library and/or graveyard for a card named Sunlit Hoplite
//!     and put it onto the battlefield. If you search your library this way,
//!     shuffle.
//!     GAP: search BY NAME (ObjectFilter has no name predicate) across two
//!     zones (library and/or graveyard) is not expressible.
//! −8: Until end of turn, creatures you control gain flying and get +X/+X,
//!     where X is your devotion to white.
//!     GAP: board-wide keyword grant (gain flying) combined with a dynamic
//!     +X/+X is not expressible — Anthem carries no keyword grant and there
//!     is no "creatures you control gain <keyword>" effect on the surface.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Elspeth, Undaunted Hero");
    let elspeth = reg.interner_mut().intern("Elspeth");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(elspeth);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Put a +1/+1 counter on each of up to two target creatures.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "\u{2212}2: Search your library and/or graveyard for a card named Sunlit Hoplite and put it onto the battlefield. If you search your library this way, shuffle.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "\u{2212}8: Until end of turn, creatures you control gain flying and get +X/+X, where X is your devotion to white.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 8)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eight,
            }),
    )
}

fn plus_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            }),
            _ => None,
        })
        .collect()
}

fn minus_two(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: search BY NAME across library and/or graveyard is not expressible
    // (ObjectFilter has no name predicate).
    Vec::new()
}

fn minus_eight(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: board-wide "gain flying" keyword grant + dynamic +X/+X (devotion)
    // is not expressible.
    Vec::new()
}
