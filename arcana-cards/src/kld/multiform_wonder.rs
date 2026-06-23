//! Multiform Wonder — `{5}` 3/3 Artifact Creature — Construct.
//!
//! * When this creature enters, you get {E}{E}{E} (three energy counters).
//! * Pay {E}: This creature gains your choice of flying, vigilance, or lifelink
//!   until end of turn.
//! * Pay {E}: This creature gets +2/-2 or -2/+2 until end of turn.
//!
//! The ETB energy gain is wired as a triggered ability. Both activated
//! abilities are GAP'd: `ActivationCost` has no energy ("Pay {E}") cost field,
//! so the activation cost is inexpressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Multiform Wonder");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = arcana_core::types::SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
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
            }),
        // GAP: "Pay {E}: gains your choice of flying, vigilance, or lifelink" —
        // ActivationCost has no energy-payment field, so the activation is
        // inexpressible (and the modal keyword choice has no Effect form).
        // GAP: "Pay {E}: gets +2/-2 or -2/+2" — same energy-cost gap plus a
        // resolution-time +2/-2-or-(-2/+2) choice with no Effect form.
    )
}

fn etb_gain_energy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainEnergy { player: trig.controller, amount: 3 }]
}
