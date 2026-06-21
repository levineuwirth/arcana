//! Felothar, Dawn of the Abzan — `{W}{B}{G}` 3/3 Legendary Human Warrior with
//! Trample.
//! Whenever Felothar enters or attacks, you may sacrifice a nonland permanent.
//! When you do, put a +1/+1 counter on each creature you control.
//!
//! Modeled as two triggers (enters, attacks), each sacrificing a nonland
//! permanent and then putting a +1/+1 counter on each creature you control.
//! The "you may" optionality and the reflexive "when you do" sub-trigger are a
//! documented fidelity simplification (sacrifice is forced here).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Felothar, Dawn of the Abzan");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{B}{G}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: sacrifice_then_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: sacrifice_then_counters,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn sacrifice_then_counters(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let nonland =
        ObjectFilter::permanent().without_types(TypeLine::LAND.into());
    let creature_ids = script::ids_matching(
        state,
        &ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        trig.controller,
    );
    let mut effects = vec![Effect::Sacrifice {
        player: trig.controller,
        filter: nonland,
        count: 1,
    }];
    for id in creature_ids {
        effects.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    vec![Effect::Sequence(effects)]
}
