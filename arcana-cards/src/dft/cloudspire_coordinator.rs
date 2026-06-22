//! Cloudspire Coordinator — `{R}{W}` 3/1 Creature — Human Pilot.
//!
//! Oracle:
//! * When this creature enters, scry 2.
//! * {T}: Create X 1/1 colorless Pilot creature tokens, where X is the number
//!   of Mounts and/or Vehicles that entered the battlefield under your control
//!   this turn. The tokens have "This token saddles Mounts and crews Vehicles
//!   as though its power were 2 greater."
//!
//! Decomposition: one ETB trigger (scry 2). The "Scry" Scryfall entry is an
//! ability word, not a keyword — no `keywords` entry.
//!
//! GAP: the {T} token-making ability has a dynamic X ("number of Mounts and/or
//!      Vehicles that entered under your control this turn") with no script
//!      helper to count entered-this-turn permanents by subtype — omitted whole
//!      rather than emitting a fixed/wrong count.

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
    let name = reg.interner_mut().intern("Cloudspire Coordinator");
    let human = reg.interner_mut().intern("Human");
    let pilot = reg.interner_mut().intern("Pilot");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(pilot);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_scry,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_scry(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Scry {
        player: trig.controller,
        count: 2,
    }]
}
