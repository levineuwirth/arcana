//! Cyclops Superconductor — `{1}{U}{R}` 2/2 Cyclops Wizard.
//! Prowess.
//! When this creature enters, you get {E}{E}{E} (three energy counters).
//! When this creature dies, you may pay {E}{E}{E}. When you do, this
//! creature deals damage equal to its power to any target.
//!
//! Prowess is not in the usable keyword surface for this card class —
//! GAP'd (see below). The ETB energy-gain is wired. The dies ability is
//! GAP'd: spending energy as a cost/gate is not modeled.

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
    let name = reg.interner_mut().intern("Cyclops Superconductor");
    let cyclops = reg.interner_mut().intern("Cyclops");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyclops);
    subtypes.0.insert(wizard);

    // GAP: keyword Prowess is not in the usable keyword surface for this
    // card class — emitting keywords: vec![].
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_gain_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // GAP: dies ability — "you may pay {E}{E}{E}, then deal damage
            // equal to power to any target" requires spending energy as a
            // cost/gate, which is not modeled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_pay_energy_gap,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_gain_energy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainEnergy { player: trig.controller, amount: 3 }]
}

fn dies_pay_energy_gap(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: spending energy as a cost ("you may pay {E}{E}{E}") is not a
    // modeled gate; the reflexive damage cannot be expressed.
    Vec::new()
}
