//! Shipwreck Moray — `{3}{U}` 0/5 blue Fish.
//!
//! Oracle:
//! * "When this creature enters, you get {E}{E}{E}{E} (four energy
//!   counters)." — an ETB trigger granting four energy via
//!   `Effect::GainEnergy`.
//! * "Pay {E}: This creature gets +2/-2 until end of turn." — GAP: paying
//!   energy as an activation cost is not an expressible cost (energy-spend
//!   is not an ActivationCost field nor an OptionalPaymentKind), so the
//!   whole activated ability is omitted (a costless +2/-2 pump would be
//!   unfaithful).

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
    let name = reg.interner_mut().intern("Shipwreck Moray");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    // GAP: activated — "Pay {E}: this creature gets +2/-2" (no energy-spend
    // cost field).
    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_energy,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_energy(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::GainEnergy {
        player: trig.controller,
        amount: 4,
    }]
}
