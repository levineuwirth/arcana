//! Ebony Owl Netsuke — `{2}` artifact (Saviors of Kamigawa, 2005).
//! "At the beginning of each opponent's upkeep, if that player has seven
//! or more cards in hand, this artifact deals 4 damage to that player."
//! StepBegins(Upkeep, Opponent) trigger; "that player" is read as the
//! opponent (exact in two-player games). The intervening-if gates on the
//! UPKEEP player's hand, which the intervening-if signature (source +
//! controller only) cannot reference — the check is performed in the
//! effect body instead (resolution-time only; a documented timing
//! fidelity gap).

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
    let name = reg.interner_mut().intern("Ebony Owl Netsuke");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
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
                // GAP: intervening-if "if that player has seven or more
                // cards in hand" gates on the upkeep player, which the
                // intervening-if fn signature cannot reference; checked in
                // the effect body instead.
                intervening_if: None,
                effect: ping_full_hand,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            },
        ),
    )
}

fn ping_full_hand(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = script::opponents(state, trig.controller);
    let Some(&p) = opponents.first() else {
        return Vec::new();
    };
    if script::hand_size(state, p) < 7 {
        return Vec::new();
    }
    vec![Effect::DealDamage {
        target: DamageTarget::Player(p),
        amount: 4,
        source: trig.source,
    }]
}
