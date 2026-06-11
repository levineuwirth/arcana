//! Oversold Cemetery — `{1}{B}` enchantment (Onslaught, 2002).
//! "At the beginning of your upkeep, if you have four or more creature
//! cards in your graveyard, you may return target creature card from your
//! graveyard to your hand."
//!
//! Upkeep trigger with a graveyard-card target. The intervening-if needs a
//! FILTERED graveyard count (creature cards only) — the `conditions`
//! helpers only count whole graveyards, so the gate is a GAP and the
//! trigger fires unconditionally. The "you may" is resolved as a yes.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Oversold Cemetery");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
                // GAP: intervening-if "if you have four or more CREATURE
                // cards in your graveyard" — conditions::graveyard_at_least
                // counts all cards, no creature-filtered graveyard predicate
                // exists; firing unconditionally.
                intervening_if: None,
                effect: return_creature_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            },
        ),
    )
}

/// "…you may return target creature card from your graveyard to your hand."
fn return_creature_card(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::ReturnFromGraveyardToHand { target: *id }]
}
