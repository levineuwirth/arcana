//! Monitor Monitor — `{2}{U}{U}` 2/5 blue Human Employee.
//!
//! "When this creature enters, open an Attraction.
//!  Once each turn, you may pay {1} to reroll one or more dice you rolled."
//!
//! "Open an Attraction" (Scryfall keyword) and the Attraction-deck mechanic are
//! Unfinity/acorn content with no engine model — there is no `Effect` for
//! opening an Attraction, so the ETB trigger is GAP'd.
//!
//! The "pay {1} to reroll dice" ability depends on the dice-rolling subsystem,
//! which the engine does not model (no reroll Effect) — GAP'd entirely.

// GAP (keyword): "Open an Attraction" — Attractions are not modeled.
// GAP (trigger): "open an Attraction" — no Effect for opening an Attraction.
// GAP (ability): "Once each turn, you may pay {1} to reroll one or more dice you
// rolled." — dice rolling / rerolling is not modeled (no reroll Effect).

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
    let name = reg.interner_mut().intern("Monitor Monitor");
    let human = reg.interner_mut().intern("Human");
    let employee = reg.interner_mut().intern("Employee");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(employee);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_open_attraction,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_open_attraction(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: opening an Attraction is not modeled.
    Vec::new()
}
