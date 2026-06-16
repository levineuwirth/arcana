//! Dunerider Outlaw — `{B}{B}` 1/1 Human Rebel Rogue.
//! Protection from green; "At the beginning of each end step, if this
//! creature dealt damage to an opponent this turn, put a +1/+1 counter on it."
//!
//! GAP (keyword): Protection from green is not in the usable KeywordAbility
//! surface — emit `keywords: vec![]`.
//! GAP (trigger): the end-step trigger is gated by an intervening-if
//! ("if this creature dealt damage to an opponent this turn"); there is no
//! conditions:: predicate or trig accessor for "this source dealt damage to
//! an opponent this turn", so the whole ability is omitted rather than
//! firing unconditionally.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dunerider Outlaw");
    let human = reg.interner_mut().intern("Human");
    let rebel = reg.interner_mut().intern("Rebel");
    let rogue = reg.interner_mut().intern("Rogue");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(rebel);
    subtypes.0.insert(rogue);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::Any,
            },
            // GAP (intervening-if): no predicate for "this creature dealt
            // damage to an opponent this turn" — left unconditional but the
            // effect is GAP'd to a no-op to avoid a materially wrong card.
            intervening_if: None,
            effect: end_step_counter,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn end_step_counter(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "if this creature dealt damage to an opponent this turn" gate is
    // unexpressible; without it the counter would be added every end step,
    // which is wrong, so the effect is omitted.
    let _ = CounterKind::PlusOnePlusOne;
    Vec::new()
}
