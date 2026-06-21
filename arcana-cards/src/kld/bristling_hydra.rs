//! Bristling Hydra — `{2}{G}{G}` 4/3 Hydra.
//! "When this creature enters, you get {E}{E}{E} (three energy counters).
//!  Pay {E}{E}{E}: Put a +1/+1 counter on this creature. It gains hexproof
//!  until end of turn."
//!
//! 1. ETB trigger → `Effect::GainEnergy` 3 (expressible).
//! 2. GAP: the "Pay {E}{E}{E}:" activated ability — spending energy as an
//!    activation cost is not a cost field in the demonstrated ActivationCost,
//!    so the ability cannot be paid for. Wiring its +1/+1-counter + hexproof
//!    payload with no cost would be a free combat trick, materially wrong, so
//!    the whole activated ability is omitted.

use arcana_core::effects::Effect;
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
    let name = reg.interner_mut().intern("Bristling Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_gain_energy,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_gain_energy(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainEnergy {
        player: trig.controller,
        amount: 3,
    }]
}
