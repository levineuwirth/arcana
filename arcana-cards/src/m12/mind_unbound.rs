//! Mind Unbound — `{4}{U}{U}` enchantment (Theros, 2013).
//! "At the beginning of your upkeep, put a lore counter on this
//! enchantment, then draw a card for each lore counter on this
//! enchantment."
//!
//! Counter-accumulation enchantment: a your-upkeep `StepBegins` trigger
//! adds one `lore` counter (`CounterKind::Lore`), then draws one card
//! per lore counter on this object. The count is read via
//! `script::source_counter_count`, which observes state BEFORE this
//! resolution's own `AddCounters` applies — so we add the printed `+1`
//! (the just-placed lore counter counts toward the draw).

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Mind Unbound");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
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
                effect: upkeep_lore_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…put a lore counter on this enchantment, then draw a card for each
/// lore counter on this enchantment." The just-placed lore counter
/// counts toward the draw, so `+1` past the pre-resolution count.
fn upkeep_lore_draw(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::source_counter_count(
        state,
        trig.source,
        CounterKind::Lore,
    ) + 1;
    vec![
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::Lore,
            count: 1,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: n,
        },
    ]
}
