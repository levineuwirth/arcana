//! Thriving Skyclaw — `{2}{R}{R}` 3/2 Creature — Cat Dragon.
//!
//! Flying.
//! When this creature enters, you get {E}{E}{E} (three energy counters).
//! Whenever this creature attacks, you may pay {E}{E}{E}. If you do, put a
//! +1/+1 counter on it.
//!
//! # Decomposition
//! * Keyword line — `Flying`.
//! * "When this creature enters, you get {E}{E}{E}" → ETB trigger (id 1)
//!   producing 3 energy via `Effect::GainEnergy`.
//! * "Whenever this creature attacks, you may pay {E}{E}{E}. If you do, put a
//!   +1/+1 counter on it" → attack trigger (id 2). GAP: the optional cost here
//!   is an ENERGY spend, and `OptionalPaymentKind` has no energy cost (only
//!   Mana / Life / Sacrifice / Discard), so the conditional payment cannot be
//!   expressed and the counter is left unwired.

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
    let name = reg.interner_mut().intern("Thriving Skyclaw");
    let cat = reg.interner_mut().intern("Cat");
    let dragon = reg.interner_mut().intern("Dragon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    subtypes.0.insert(dragon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: gain_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: maybe_pay_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn gain_energy(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainEnergy {
        player: trig.controller,
        amount: 3,
    }]
}

fn maybe_pay_energy(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {E}{E}{E}. If you do, put a +1/+1 counter on it" —
    // energy is not a payable cost (OptionalPaymentKind has Mana/Life/Sacrifice/
    // Discard but no energy spend).
    Vec::new()
}
