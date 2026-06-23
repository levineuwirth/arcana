//! Cephalid Inkmage — `{2}{U}` 2/2 Octopus Wizard.
//!
//! * "When this creature enters, surveil 3." → ETB triggered ability emitting
//!   `Effect::Surveil { count: 3 }`.
//! * "Threshold — This creature can't be blocked as long as there are seven
//!   or more cards in your graveyard." → a STATIC continuous ability gated on
//!   a graveyard threshold. The engine has `Effect::CantBeBlocked` but no
//!   continuous "while you have ≥7 cards in graveyard" static-installation
//!   primitive in the documented surface, so the gated static is GAP'd.

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
    let name = reg.interner_mut().intern("Cephalid Inkmage");
    let octopus = reg.interner_mut().intern("Octopus");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(octopus);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
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
                effect: etb_surveil_3,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
        // GAP: static "Threshold — can't be blocked while you have 7+ cards in
        // graveyard" — no documented gated continuous-static install primitive.
    )
}

fn etb_surveil_3(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Surveil { player: trig.controller, count: 3 }]
}
