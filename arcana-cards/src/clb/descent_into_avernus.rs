//! Descent into Avernus — `{2}{R}` enchantment (Adventures in the
//! Forgotten Realms, 2021).
//! "At the beginning of your upkeep, put two descent counters on this
//! enchantment. Then each player creates X Treasure tokens and this
//! enchantment deals X damage to each player, where X is the number of
//! descent counters on this enchantment."
//!
//! Counter-accumulation enchantment: a your-upkeep `StepBegins` trigger
//! adds two `descent` counters (`CounterKind::Named`), then — with
//! X = the number of descent counters now on this object — every player
//! mints X Treasure tokens and this enchantment deals X damage to every
//! player. X is read via `script::source_counter_count`, which observes
//! state BEFORE this resolution's own `AddCounters` applies, so we add
//! the printed `+2` (the just-placed counters count toward X).
//!
//! Each player's Treasures are minted via `Effect::CreateCommodityToken`
//! (its `controller` field lets us target every player, not just us) and
//! the damage is one `Effect::DealDamage` per player, both fanned out
//! over `script::all_players`.

use arcana_core::effects::{CommodityToken, Effect};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Descent into Avernus");
    let _descent = reg.interner_mut().intern("descent");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
                effect: upkeep_descent,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…put two descent counters on this enchantment. Then each player
/// creates X Treasure tokens and this enchantment deals X damage to each
/// player, where X is the number of descent counters on this enchantment."
/// The two just-placed counters count toward X, so `+2` past the
/// pre-resolution count.
fn upkeep_descent(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let descent = reg.interner().lookup("descent")
        .expect("descent interned during register()");

    let x = script::source_counter_count(
        state,
        trig.source,
        CounterKind::Named(descent),
    ) + 2;

    let mut effects = vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Named(descent),
        count: 2,
    }];
    for p in script::all_players(state) {
        effects.push(Effect::CreateCommodityToken {
            controller: p,
            kind: CommodityToken::Treasure,
            count: x,
        });
        effects.push(Effect::DealDamage {
            target: DamageTarget::Player(p),
            amount: x,
            source: trig.source,
        });
    }
    vec![Effect::Sequence(effects)]
}
