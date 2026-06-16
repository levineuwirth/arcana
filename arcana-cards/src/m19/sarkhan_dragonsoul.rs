//! Sarkhan, Dragonsoul — `{4}{R}{R}` Legendary Planeswalker — Sarkhan,
//! starting loyalty 7. Mono-red.
//!
//! Oracle:
//! +2: Sarkhan deals 1 damage to each opponent and each creature your
//!     opponents control.
//! −3: Sarkhan deals 4 damage to target player or planeswalker.
//! −9: Search your library for any number of Dragon creature cards, put them
//!     onto the battlefield, then shuffle.
//!
//! # Scope
//! * +2 — modeled: 1 damage to each opponent and to each creature an opponent
//!   controls (resolved over the current board).
//! * −3 — partial: 4 damage to the chosen target. Modeled with an any-target
//!   requirement (the engine has no "player or planeswalker"-only filter), so
//!   it slightly over-includes creatures.
//! * −9 — partial: search your library for a Dragon creature card and put it
//!   onto the battlefield, then shuffle. GAP: "any number" collapses to one
//!   (Search fetches a single match).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, ObjectOrPlayer, TargetChoice,
    TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sarkhan, Dragonsoul");
    let sub = reg.interner_mut().intern("Sarkhan");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);
    let _ = reg.interner_mut().intern("Dragon");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(7),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Sarkhan deals 1 damage to each opponent and each \
                       creature your opponents control.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_sweep,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Sarkhan deals 4 damage to target player or \
                       planeswalker.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−9: Search your library for any number of Dragon \
                       creature cards, put them onto the battlefield, then \
                       shuffle.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_search,
            }),
    )
}

/// `+2:` 1 damage to each opponent and each creature opponents control.
fn plus_two_sweep(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = state
        .opponents_of(ctx.controller)
        .map(|p| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(p),
            amount: 1,
        })
        .collect();
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
    for id in script::ids_matching(state, &filter, ctx.controller) {
        effects.push(Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(id),
            amount: 1,
        });
    }
    effects
}

/// `−3:` 4 damage to the chosen target.
fn minus_three_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
            DamageTarget::Object(*id)
        }
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
            DamageTarget::Player(*p)
        }
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 4,
    }]
}

/// `−9:` search your library for a Dragon creature card, put it onto the
/// battlefield, then shuffle ("any number" collapses to one).
fn minus_nine_search(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let dragon = reg.interner().lookup("Dragon").unwrap_or_default();
    let filter = ObjectFilter::creature().with_subtype_sym(dragon);
    vec![Effect::Search {
        player: ctx.controller,
        zone: Zone::Library(ctx.controller),
        filter,
        destination: Zone::Battlefield,
        reveal: false,
    }]
}
