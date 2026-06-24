//! Pumpkin Bombs — `{1}{R}` artifact (Mercadian Masques, 1999).
//! "{T}, Discard two cards: Draw three cards, then put a fuse counter on
//! this artifact. It deals damage equal to the number of fuse counters
//! on it to target opponent. They gain control of this artifact."
//!
//! A red non-creature artifact with one activated ability:
//!   * cost: {T} + discard two cards (discard_other with
//!     discard_other_count: 2),
//!   * targets target opponent (TargetFilter::Player, Opponent),
//!   * effect: draw three, add a fuse counter, deal damage equal to the
//!     fuse counters now on it (dynamic via script::source_counter_count
//!     on a Named("fuse") counter), then ChangeControl to that opponent.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pumpkin Bombs");
    // Pre-intern the (non-standard) fuse counter name so the resolver
    // can rebuild the same CounterKind::Named handle.
    let _fuse = reg.interner_mut().intern("fuse");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{T}, Discard two cards: Draw three cards, then put a fuse \
                   counter on this artifact. It deals damage equal to the number \
                   of fuse counters on it to target opponent. They gain control \
                   of this artifact."
                .into(),
            cost: ActivationCost {
                tap: true,
                discard_other: Some(ObjectFilter::default()),
                discard_other_count: 2,
                ..ActivationCost::default()
            },
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Player,
                count: TargetCount::Exactly(1),
                controller: Some(ControllerConstraint::Opponent),
            }],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pumpkin_bombs_effect,
        }),
    )
}

fn pumpkin_bombs_effect(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(opp)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let fuse = reg
        .interner()
        .lookup("fuse")
        .map(CounterKind::Named)
        .unwrap_or(CounterKind::Charge);
    // Fuse counters AFTER this activation adds one.
    let fuses = script::source_counter_count(state, ctx.source, fuse) + 1;
    vec![Effect::Sequence(vec![
        Effect::DrawCards {
            player: ctx.controller,
            count: 3,
        },
        Effect::AddCounters {
            target: ctx.source,
            kind: fuse,
            count: 1,
        },
        Effect::DealDamage {
            target: DamageTarget::Player(*opp),
            amount: fuses,
            source: ctx.source,
        },
        Effect::ChangeControl {
            target: ctx.source,
            new_controller: *opp,
        },
    ])]
}
