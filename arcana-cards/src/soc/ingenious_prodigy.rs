//! Ingenious Prodigy — `{X}{U}` 0/1 blue Human Wizard with Skulk.
//! This creature enters with X +1/+1 counters on it. (GAP — no enters-with-X primitive.)
//! At the beginning of your upkeep, if this creature has one or more +1/+1
//! counters on it, you may remove a +1/+1 counter from it. If you do, draw a card.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::mana::ManaCost;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::conditions;
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ingenious Prodigy");
    let human = reg.interner_mut().intern("Human");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Skulk],
        ..Default::default()
    };

    // GAP: "This creature enters with X +1/+1 counters on it." — no primitive
    // ties an enters-with counter count to the spell's chosen X.
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::StepBegins {
                    step: Step::Upkeep,
                    whose: ControllerConstraint::You,
                },
                intervening_if: Some(if_has_plus_one_counter),
                effect: remove_counter_draw,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Intervening-if: this creature has one or more +1/+1 counters on it.
fn if_has_plus_one_counter(
    s: &GameState,
    src: ObjectId,
    _you: PlayerId,
    _reg: &CardRegistry,
) -> bool {
    conditions::source_counters_at_least(s, src, CounterKind::PlusOnePlusOne, 1)
}

/// Remove a +1/+1 counter, then draw a card. NOTE: the printed "you may"
/// optional is modeled as mandatory — there is no optional non-mana/life cost
/// gate (OptionalPaymentKind covers only Mana/Life), so the remove-then-draw is
/// applied directly. The intervening-if already gates on a counter being present.
fn remove_counter_draw(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::RemoveCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::DrawCards {
            player: trig.controller,
            count: 1,
        },
    ]
}
