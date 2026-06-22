//! Slumbering Dragon — `{R}` 3/3 Dragon.
//! Flying.
//! This creature can't attack or block unless it has five or more +1/+1
//! counters on it.
//! Whenever a creature attacks you or a planeswalker you control, put a
//! +1/+1 counter on this creature.
//!
//! Flying is a usable keyword. The "can't attack or block unless it has
//! five +1/+1 counters" static is GAP'd (no counter-gated attack/block
//! restriction primitive). The attack trigger is wired as
//! `CreatureAttacks { creature an opponent controls }` (the closest match for
//! "attacks you or a planeswalker you control"; the defending-player /
//! planeswalker scoping is a fidelity GAP) → +1/+1 counter on this creature.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Slumbering Dragon");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dragon);

    // GAP: "This creature can't attack or block unless it has five or more
    // +1/+1 counters on it." — counter-gated attack/block restriction static,
    // no expressible primitive.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::CreatureAttacks {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                },
                intervening_if: None,
                effect: grow,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn grow(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
