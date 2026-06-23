//! Cyclone — `{2}{G}{G}` enchantment (Arabian Nights, 1993).
//! "At the beginning of your upkeep, put a wind counter on this
//! enchantment, then sacrifice this enchantment unless you pay {G} for
//! each wind counter on it. If you pay, this enchantment deals damage
//! equal to the number of wind counters on it to each creature and each
//! player."
//!
//! Counter-accumulation enchantment: a your-upkeep `StepBegins` trigger
//! adds one `wind` counter (`CounterKind::Named`). N = the wind counters
//! now on this object — read via `script::source_counter_count`, which
//! observes state BEFORE this resolution's own `AddCounters`, so we add
//! the printed `+1` (the just-placed counter counts toward N).
//!
//! The "sacrifice unless you pay {G} per wind counter" gate is the
//! "Z unless you pay X" polarity of `Effect::OptionalPayment`:
//!   * the per-counter mana tax `{G}×N` is built at resolution as a
//!     dynamic `ManaCost::parse(&"{G}".repeat(N))` (the same dynamic-cost
//!     idiom landed for Esper Sentinel's `{X}` gate);
//!   * `then` (you paid) deals N damage to each creature and each player —
//!     one `DealDamage` per creature id (via `script::ids_matching`) and
//!     one per player (via `script::all_players`), wrapped in a `Sequence`;
//!   * `else_effect` (you didn't pay) sacrifices this enchantment.
//!
//! FIDELITY NOTE: there is no immediate self-sacrifice `Effect`, so the
//! sacrifice clause uses `Effect::DestroyPermanent { target: trig.source }`
//! as the established stand-in (cf. Flame-Kin War Scout) — it removes the
//! enchantment now; the only divergence is destruction- vs sacrifice-
//! specific replacement/trigger semantics.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cyclone");
    let _wind = reg.interner_mut().intern("wind");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: upkeep_wind,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…put a wind counter on this enchantment, then sacrifice this
/// enchantment unless you pay {G} for each wind counter on it. If you pay,
/// this enchantment deals damage equal to the number of wind counters on
/// it to each creature and each player." The just-placed wind counter
/// counts toward N, so `+1` past the pre-resolution count.
fn upkeep_wind(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let wind = reg.interner().lookup("wind")
        .expect("wind interned during register()");

    let n = script::source_counter_count(
        state,
        trig.source,
        CounterKind::Named(wind),
    ) + 1;

    // "deals N damage to each creature and each player" — one DealDamage
    // per creature id (ForEach does not substitute per-id), then one per
    // player.
    let mut payoff: Vec<Effect> = script::ids_matching(
        state,
        &ObjectFilter::creature(),
        trig.controller,
    )
    .into_iter()
    .map(|id| Effect::DealDamage {
        target: DamageTarget::Object(id),
        amount: n,
        source: trig.source,
    })
    .collect();
    for p in script::all_players(state) {
        payoff.push(Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: n,
            source: trig.source,
        });
    }

    // "{G} for each wind counter on it" — N green pips, built dynamically.
    let tax = ManaCost::parse(&"{G}".repeat(n as usize))
        .unwrap_or_else(|_| ManaCost::parse("{0}").expect("valid cost"));

    vec![
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Named(wind),
            count: 1,
        },
        // "sacrifice this enchantment UNLESS you pay {G}×N. If you pay, [payoff]."
        Effect::OptionalPayment {
            chooser: trig.controller,
            cost: OptionalPaymentKind::Mana(tax),
            then: Box::new(Effect::Sequence(payoff)),
            // no immediate SacrificeSelf — DestroyPermanent on the source is
            // the established stand-in for the self-sacrifice clause.
            else_effect: Some(Box::new(Effect::DestroyPermanent {
                target: trig.source,
            })),
        },
    ]
}
