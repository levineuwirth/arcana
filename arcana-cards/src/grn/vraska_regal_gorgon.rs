//! Vraska, Regal Gorgon — `{5}{B}{G}` Legendary Planeswalker — Vraska,
//! starting loyalty 6 — colors B, G.
//!
//! Oracle text:
//! * `+2`: Put a +1/+1 counter on up to one target creature. That creature
//!   gains menace until end of turn. — `AddCounters` + `GrantKeyword(Menace,
//!   EndOfTurn)` on the (optional) target.
//! * `−3`: Destroy target creature. — `Effect::DestroyPermanent`.
//! * `−10`: For each creature card in your graveyard, put a +1/+1 counter on
//!   each creature you control. — count creature cards in your graveyard
//!   (`script::graveyard_matching`), then `ForEach` over your creatures with an
//!   inner `AddCounters` of that count.
//!
//! # Rules references
//! * CR 606 — loyalty abilities.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vraska, Regal Gorgon");
    let vraska = reg.interner_mut().intern("Vraska");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vraska);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Put a +1/+1 counter on up to one target creature. \
                       That creature gains menace until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_counter_menace,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy target creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−10: For each creature card in your graveyard, put a \
                       +1/+1 counter on each creature you control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten_mass_counters,
            }),
    )
}

/// `+2`: +1/+1 counter on up to one target creature, that creature gains
/// menace until end of turn.
fn plus_two_counter_menace(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => vec![
            Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            },
            Effect::GrantKeyword {
                target: *id,
                keyword: KeywordAbility::Menace,
                duration: Duration::EndOfTurn,
            },
        ],
        _ => Vec::new(),
    }
}

/// `−3`: destroy target creature.
fn minus_three_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    match ctx.targets.targets.first() {
        Some(TargetChoice::Object(id)) => {
            vec![Effect::DestroyPermanent { target: *id }]
        }
        _ => Vec::new(),
    }
}

/// `−10`: for each creature card in your graveyard, put a +1/+1 counter on
/// each creature you control.
fn minus_ten_mass_counters(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let gy_creatures = ObjectFilter::default().with_types(TypeLine::CREATURE.into());
    let n = script::graveyard_matching(state, &gy_creatures, ctx.controller, ctx.controller);
    if n == 0 {
        return Vec::new();
    }
    let your_creatures = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &your_creatures, ctx.controller);
    if ids.is_empty() {
        return Vec::new();
    }
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::AddCounters {
            target: NULL_OBJECT_ID,
            kind: CounterKind::PlusOnePlusOne,
            count: n,
        }),
    }]
}
