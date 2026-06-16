//! Oko, the Ringleader — `{2}{G}{U}` legendary planeswalker, starting
//! loyalty 4. Subtype Oko, colors green + blue.
//!
//! Oracle text:
//! * At the beginning of combat on your turn, Oko becomes a copy of up
//!   to one target creature you control until end of turn, except he
//!   has hexproof.
//! * `+1`: Draw two cards. If you've committed a crime this turn,
//!   discard a card. Otherwise, discard two cards.
//! * `−1`: Create a 3/3 green Elk creature token.
//! * `−5`: For each other nonland permanent you control, create a token
//!   that's a copy of that permanent.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 — loyalty abilities (the `+1` / `−1` / `−5` lines).
//! * CR 704.5i — a 0-loyalty planeswalker is sacrificed as an SBA.
//!
//! # Scope / gaps
//!
//! * The combat trigger ("Oko becomes a copy of up to one target
//!   creature you control … except he has hexproof") is GAP'd. The
//!   demonstrated `Effect::CopyPermanent` mints a TOKEN copy of the
//!   target; there is no "this planeswalker becomes a copy of another
//!   object" re-skin primitive. The trigger shell + its target
//!   requirement are still declared so the catalog records the ability.
//! * `+1` — the DRAW (two cards) is implemented. The conditional
//!   discard depends on "have you committed a crime this turn", which
//!   has no predicate in the demonstrated surface; the discard count is
//!   condition-dependent (1 vs 2) so it is GAP'd rather than hardcoded.
//! * `−1` — fully implemented (3/3 green Elk token).
//! * `−5` — fully implemented: each other nonland permanent you control
//!   gets a token copy via `Effect::CopyPermanent`.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oko, the Ringleader");
    let oko = reg.interner_mut().intern("Oko");
    // Interned here so the −1 Elk token's name/subtype symbol exists in
    // the interner before the resolver looks it up.
    let _elk = reg.interner_mut().intern("Elk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(oko);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // At the beginning of combat on your turn, Oko becomes a
            // copy of up to one target creature you control … hexproof.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::Combat,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: combat_becomes_copy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature()
                            .controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
            })
            // +1: Draw two cards. If you've committed a crime this turn,
            // discard a card. Otherwise, discard two cards.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Draw two cards. If you've committed a crime \
                       this turn, discard a card. Otherwise, discard two \
                       cards."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_draw,
            })
            // −1: Create a 3/3 green Elk creature token.
            .with_activated_ability(ActivatedAbilityDef {
                text: "−1: Create a 3/3 green Elk creature token.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_one_elk,
            })
            // −5: For each other nonland permanent you control, create a
            // token that's a copy of that permanent.
            .with_activated_ability(ActivatedAbilityDef {
                text: "−5: For each other nonland permanent you control, \
                       create a token that's a copy of that permanent."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five_copies,
            }),
    )
}

/// Combat trigger: "Oko becomes a copy of up to one target creature you
/// control until end of turn, except he has hexproof."
fn combat_becomes_copy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a copy of target creature" overwrites the SOURCE
    // planeswalker's characteristics. The demonstrated
    // `Effect::CopyPermanent` mints a TOKEN copy of the target — there
    // is no becomes-a-copy-onto-self primitive in the surface, and the
    // "except he has hexproof" rider is likewise unexpressible.
    Vec::new()
}

/// `+1`: Draw two cards (conditional discard GAP'd).
fn plus_one_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "If you've committed a crime this turn, discard a card.
    // Otherwise, discard two cards." There is no "committed a crime"
    // predicate in the demonstrated surface, and the discard count is
    // condition-dependent (1 vs 2) — emitting either would be wrong, so
    // the conditional discard is omitted. The draw is faithful.
    vec![Effect::DrawCards {
        player: ctx.controller,
        count: 2,
    }]
}

/// `−1`: Create a 3/3 green Elk creature token.
fn minus_one_elk(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let elk = reg.interner().lookup("Elk").unwrap_or(0);
    let mut subtypes = SubtypeSet::default();
    if elk != 0 {
        subtypes.0.insert(elk);
    }
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: arcana_core::effects::TokenDefinition {
            name: elk,
            colors: ColorSet::green(),
            types: TypeLine::CREATURE.into(),
            subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

/// `−5`: For each other nonland permanent you control, create a token
/// that's a copy of that permanent.
fn minus_five_copies(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::permanent()
        .without_types(TypeLine::LAND.into())
        .controlled_by(ControllerConstraint::You);
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        // "each OTHER nonland permanent" — exclude Oko itself.
        .filter(|&id| id != ctx.source)
        .map(|id| Effect::CopyPermanent { target: id })
        .collect()
}
