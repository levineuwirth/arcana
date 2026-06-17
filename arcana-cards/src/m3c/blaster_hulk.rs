//! Blaster Hulk — `{6}{R}{R}` 8/8 Artifact Creature — Pirate (red).
//!
//! * "This spell costs {1} less to cast for each {E} you've paid or lost this
//!   turn." — a cast cost reduction; no cost-modification primitive. GAP'd.
//! * Haste.
//! * "Whenever this creature attacks, you get {E}{E}, then you may pay eight
//!   {E}. When you do, this creature deals 8 damage divided as you choose
//!   among up to eight targets." — SelfAttacks trigger grants 2 energy. GAP:
//!   spending energy is not a cost field, so the optional "pay eight {E}" and
//!   its reflexive divided-damage payoff are not expressible.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Blaster Hulk");
    let pirate = reg.interner_mut().intern("Pirate");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(pirate);

    // GAP: "{1} less per {E} paid or lost this turn" cost reduction.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(8)),
        toughness: Some(PtValue::Fixed(8)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
        id: 1,
        trigger_condition: TriggerCondition::SelfAttacks,
        intervening_if: None,
        effect: gain_energy,
        trigger_zones: vec![Zone::Battlefield],
        frequency: TriggerFrequency::EachTime,
        target_requirements: Vec::new(),
    }))
}

fn gain_energy(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: optional "pay eight {E}" + reflexive 8-divided-damage payoff —
    // energy spending is not a cost field.
    vec![Effect::GainEnergy {
        player: trig.controller,
        amount: 2,
    }]
}
