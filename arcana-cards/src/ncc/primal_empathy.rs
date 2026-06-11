//! Primal Empathy — `{1}{G}{U}` enchantment.
//! "At the beginning of your upkeep, draw a card if you control a
//! creature with the greatest power among creatures on the battlefield.
//! Otherwise, put a +1/+1 counter on a creature you control."
//!
//! // GAP: fidelity — the "otherwise" counter goes on a deterministic
//! // creature (the first matching id), not a player-chosen one.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Primal Empathy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
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
                effect: draw_or_grow,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

/// "…draw a card if you control a creature with the greatest power among
/// creatures on the battlefield. Otherwise, put a +1/+1 counter on a
/// creature you control."
fn draw_or_grow(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let all = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    let yours = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let greatest = all
        .iter()
        .map(|&id| script::power_of(state, id))
        .max()
        .unwrap_or(0);
    let your_best = yours
        .iter()
        .map(|&id| script::power_of(state, id))
        .max()
        .unwrap_or(i32::MIN);
    if !yours.is_empty() && your_best >= greatest {
        vec![Effect::DrawCards {
            player: trig.controller,
            count: 1,
        }]
    } else if let Some(&id) = yours.first() {
        // GAP: counter recipient is the first matching creature id, not a
        // player-chosen one.
        vec![Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        }]
    } else {
        Vec::new()
    }
}
