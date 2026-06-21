//! Wight — `{1}{B}` 3/2 Creature — Zombie Soldier.
//! "This creature enters tapped." — wired as an ETB trigger that taps itself.
//! "Life Drain — Whenever a creature dealt damage by this creature this turn
//! dies, create a tapped 2/2 black Zombie creature token and exile that card."
//! — the trigger watches deaths of creatures THIS creature damaged this turn (a
//! per-source damage-history filter) and exiles the dying card; neither the
//! damaged-by-this-creature filter nor the make-tapped-token + exile-the-card
//! coupling is expressible with the demonstrated API; GAP'd.

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
    let name = reg.interner_mut().intern("Wight");
    let zombie = reg.interner_mut().intern("Zombie");
    let soldier = reg.interner_mut().intern("Soldier");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(soldier);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    // GAP: "Life Drain" — death of a creature this creature damaged this turn,
    // making a tapped 2/2 black Zombie token and exiling that card; the
    // damaged-by-this-source filter and the exile-the-dying-card coupling are
    // not expressible here. Trigger omitted.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: enter_tapped,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn enter_tapped(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Tap { target: trig.source }]
}
