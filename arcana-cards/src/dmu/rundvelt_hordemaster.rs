//! Rundvelt Hordemaster — `{1}{R}` 1/1 red Goblin Warrior.
//!
//! Oracle:
//! * Other Goblins you control get +1/+1. — GAP: static continuous anthem,
//!   not a triggered/activated ability.
//! * Whenever this creature or another Goblin you control dies, exile the top
//!   card of your library; if it's a Goblin creature card you may cast it
//!   until the end of your next turn.
//!
//! The death trigger fires on any Goblin you control (this card included)
//! moving from battlefield to graveyard. The payload is modeled with
//! `Effect::ImpulseExile { count: 1 }` (exile the top card, you may play it) —
//! a documented fidelity gap: it doesn't restrict the playable card to a
//! Goblin creature, and the play window is "this turn" rather than "until end
//! of your next turn".

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rundvelt Hordemaster");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);

    let goblin_filter = script::subtype_filter(reg, "Goblin")
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    // GAP: "Other Goblins you control get +1/+1" — pure static anthem.

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: goblin_filter,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: exile_top_may_cast,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn exile_top_may_cast(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::ImpulseExile { player: trig.controller, count: 1 }]
}
