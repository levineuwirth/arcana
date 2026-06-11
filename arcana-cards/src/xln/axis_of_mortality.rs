//! Axis of Mortality — `{4}{W}{W}` enchantment.
//! "At the beginning of your upkeep, you may have two target players
//! exchange life totals."
//!
//! The exchange is computed at resolution via `script::life` and
//! emitted as two `Effect::SetLifeTotal`s. GAP: the "you may" is
//! resolved as mandatory (no free optional wrapper exists).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Axis of Mortality");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
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
                effect: exchange_life_totals,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(2),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…you may have two target players exchange life totals."
/// (GAP: "you may" resolved as mandatory.)
fn exchange_life_totals(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut players = trig.targets.targets.iter().filter_map(|t| match t {
        TargetChoice::Player(p) => Some(*p),
        _ => None,
    });
    let (Some(a), Some(b)) = (players.next(), players.next()) else {
        return Vec::new();
    };
    let life_a = script::life(state, a).max(0) as u32;
    let life_b = script::life(state, b).max(0) as u32;
    vec![
        Effect::SetLifeTotal {
            player: a,
            amount: life_b,
        },
        Effect::SetLifeTotal {
            player: b,
            amount: life_a,
        },
    ]
}
