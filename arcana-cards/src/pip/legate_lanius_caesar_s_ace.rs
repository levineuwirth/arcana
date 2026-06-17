//! Legate Lanius, Caesar's Ace — `{2}{B}{R}` 2/2 Legendary Human Soldier.
//! Decimate — When Legate Lanius enters, each opponent sacrifices a tenth of
//!   the creatures they control of their choice, rounded up.
//! Whenever an opponent sacrifices a creature, put a +1/+1 counter on Lanius.
//! GAP: "Decimate" is not in the available keyword surface (omitted from
//! keywords; the ETB effect itself is implemented).

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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Legate Lanius, Caesar's Ace");
    let human = reg.interner_mut().intern("Human");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(soldier);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: decimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::Sacrificed {
                    filter: ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent),
                },
                intervening_if: None,
                effect: gain_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn decimate(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Each opponent sacrifices ceil(their creatures / 10) creatures of their choice.
    let mut effects = Vec::new();
    for p in script::opponents(state, trig.controller) {
        let total = script::count_matching(
            state,
            &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
            p,
        );
        let count = total.div_ceil(10);
        if count > 0 {
            effects.push(Effect::Sacrifice {
                player: p,
                filter: ObjectFilter::creature(),
                count,
            });
        }
    }
    effects
}

fn gain_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
