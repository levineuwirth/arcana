//! Walking Ballista — `{X}{X}` artifact creature. Enters with X
//! +1/+1 counters; remove a counter to deal 1 damage to any target,
//! and pay `{4}` to add a counter. The canonical X-in-P/T + counter-
//! removal-as-cost stress test for Phase 2.
//!
//! # Rules references
//!
//! * CR 107.3b — X is chosen as the spell is cast; the cost paid is
//!   `2X` generic mana for `{X}{X}` (i.e. each `{X}` costs X).
//! * CR 121.6a — "enters the battlefield with X +1/+1 counters" is a
//!   replacement, not a trigger, so a 0/0 Ballista doesn't die to SBA
//!   between entering and receiving its counters. Modelled here via
//!   [`EntersWithSpec::CountersFromX`].
//! * CR 602.1b — `{4}: Put a +1/+1 counter on ~` is a plain mana cost.
//! * CR 118.12 — "Remove a +1/+1 counter from ~" is an additional
//!   cost; the engine tracks it via
//!   [`ActivationCost::remove_self_counter`].
//!
//! # Simplifications
//!
//! The original printed text adds "Walking Ballista enters the
//! battlefield with X +1/+1 counters on it" and the two activated
//! abilities. We model exactly that and nothing more — no typeline
//! subtype (Construct), because Construct-matters effects are out of
//! the Phase 2 seed scope.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, CardDefinition,
    CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Walking Ballista");
    // {X}{X} — parsed as two generic X costs; see
    // `ManaCost::with_x_expanded` for the expansion at cast time.
    let mana_cost = ManaCost::parse("{X}{X}").expect("valid cost");
    let chars = Characteristics {
        name,
        mana_cost: Some(mana_cost),
        colors: ColorSet::default(), // artifact creature; colorless
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        // Base 0/0 — counters placed via `enters_with` (CR 121.6a)
        // arrive before SBA, so it never dies on entry.
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::CountersFromX {
                kind: CounterKind::PlusOnePlusOne,
            })
            // "{4}: Put a +1/+1 counter on ~" — plain mana cost,
            // no target, applies to self.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}: Put a +1/+1 counter on Walking Ballista.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                effect: add_counter_to_self,
            })
            // "Remove a +1/+1 counter from ~: ~ deals 1 damage to any
            // target." — counter-removal-as-additional-cost; single
            // "any target" selection.
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove a +1/+1 counter from Walking Ballista: \
                       Walking Ballista deals 1 damage to any target.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::PlusOnePlusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                effect: ping_any_target,
            }),
    )
}

/// `{4}`: Put a +1/+1 counter on self.
fn add_counter_to_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}

/// Remove-counter ping: deal 1 damage to the declared target.
fn ping_any_target(
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
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 1,
    }]
}
