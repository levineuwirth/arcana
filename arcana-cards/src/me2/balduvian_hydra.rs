//! Balduvian Hydra — `{X}{R}{R}` 0/1 Hydra.
//!
//! This creature enters with X +1/+0 counters on it.
//! Remove a +1/+0 counter from this creature: Prevent the next 1
//!   damage that would be dealt to it this turn.
//! {R}{R}{R}: Put a +1/+0 counter on this creature. Activate only
//!   during your upkeep.
//!
//! The two activated abilities are wired around a named "+1/+0"
//! counter. GAPs:
//!   * "enters with X +1/+0 counters" — a dynamic enters-with-counters
//!     replacement, not expressible.
//!   * "Activate only during your upkeep" — no expressible upkeep-only
//!     activation gate; the counter-add effect IS wired.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Balduvian Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    // named counter for "+1/+0"
    let plus10 = reg.interner_mut().intern("+1/+0");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "This creature enters with X +1/+0 counters on it." —
    // dynamic enters-with-counters replacement, not expressible.
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Remove a +1/+0 counter from this creature: \
                       Prevent the next 1 damage that would be dealt \
                       to it this turn."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Named(plus10), 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: prevent_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                // GAP: "Activate only during your upkeep." — no
                // upkeep-only activation gate is expressible.
                text: "{R}{R}{R}: Put a +1/+0 counter on this creature."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{R}{R}{R}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_plus10,
            }),
    )
}

fn prevent_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(ctx.source),
        amount: Some(1),
        duration: ReplacementDuration::EndOfTurn,
    }]
}

fn add_plus10(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(plus10) = reg.interner().lookup("+1/+0") else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Named(plus10),
        count: 1,
    }]
}
