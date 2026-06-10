//! Skullcage — `{4}` artifact (Darksteel, 2004).
//! "At the beginning of each opponent's upkeep, this artifact deals 2
//! damage to that player unless they have exactly three or exactly four
//! cards in hand." StepBegins(Upkeep, Opponent); "that player" is read as
//! the opponent (exact in two-player games). The hand-size escape clause
//! references the upkeep player, which the intervening-if signature
//! cannot reach — it is checked in the effect body (resolution-time only,
//! a documented timing fidelity gap).

use arcana_core::effects::Effect;
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
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Skullcage");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(
            TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::Opponent,
                },
                // GAP: "unless they have exactly three or exactly four
                // cards in hand" gates on the upkeep player; the
                // intervening-if signature cannot reference that player,
                // so the check lives in the effect body.
                intervening_if: None,
                effect: cage_ping,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn cage_ping(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let Some(&p) = opponents.first() else {
        return Vec::new();
    };
    let hand = script::hand_size(state, p);
    if hand == 3 || hand == 4 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        target: DamageTarget::Player(p),
        amount: 2,
        source: trig.source,
    }]
}
