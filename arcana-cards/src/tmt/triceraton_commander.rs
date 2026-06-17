//! Triceraton Commander — `{X}{X}{W}{W}` 2/2 Dinosaur Soldier.
//! Flying.
//! Whenever this creature attacks, Dinosaurs you control other than this
//!   creature get +1/+1 and gain flying until end of turn.
//! When this creature enters, create X 2/2 white Dinosaur Soldier tokens.
//!   (X is unreadable from the ETB trigger — see GAP.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Triceraton Commander");
    let dinosaur = reg.interner_mut().intern("Dinosaur");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(dinosaur);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{X}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: pump_dinosaurs,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_make_tokens,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn pump_dinosaurs(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let filter = script::subtype_filter(reg, "Dinosaur").controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    let effects: Vec<Effect> = ids
        .into_iter()
        .filter(|id| *id != trig.source)
        .map(|id| Effect::Pump {
            target: id,
            power: 1,
            toughness: 1,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Flying],
        })
        .collect();
    vec![Effect::Sequence(effects)]
}

fn etb_make_tokens(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "create X 2/2 white Dinosaur Soldier tokens" — X is the value paid
    //      when this was cast, which is not reachable from the ETB trigger
    //      (PendingTrigger exposes no x_value).
    Vec::new()
}
