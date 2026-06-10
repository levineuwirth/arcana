//! Engulfing Slagwurm — `{5}{G}{G}` 7/7 green Wurm creature.
//! "Whenever this creature blocks or becomes blocked by a creature, destroy that creature.
//! You gain life equal to that creature's toughness."
//! "That creature" is recovered per direction: `script::blockers_of` for the
//! becomes-blocked trigger, `script::attackers_blocked_by` for the blocks
//! trigger (covers every paired creature, matching the oracle's
//! per-creature trigger).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Engulfing Slagwurm");
    let wurm = reg.interner_mut().intern("Wurm");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(wurm);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(7)),
        toughness: Some(PtValue::Fixed(7)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfBecomesBlocked,
                intervening_if: None,
                effect: becomes_blocked_destroy_and_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfBlocks,
                intervening_if: None,
                effect: blocks_destroy_and_gain,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn destroy_and_gain(state: &GameState, trig: &PendingTrigger, ids: Vec<ObjectId>) -> Vec<Effect> {
    ids.into_iter()
        .flat_map(|id| {
            let toughness = script::toughness_of(state, id).max(0) as u32;
            [
                Effect::DestroyPermanent { target: id },
                Effect::GainLife { player: trig.controller, amount: toughness },
            ]
        })
        .collect()
}

fn becomes_blocked_destroy_and_gain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    destroy_and_gain(state, trig, script::blockers_of(state, trig.source))
}

fn blocks_destroy_and_gain(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    destroy_and_gain(state, trig, script::attackers_blocked_by(state, trig.source))
}
