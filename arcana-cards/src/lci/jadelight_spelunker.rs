//! Jadelight Spelunker — `{X}{G}` 1/1 Creature — Merfolk Scout. Green.
//! "When this creature enters, it explores X times."
//!
//! The ETB trigger is wired, but its effect is GAP'd: "explores X times"
//! scales with the X paid to cast the creature, and the demonstrated
//! PendingTrigger surface exposes no accessor for the source's cast
//! x_value. Per the dynamic-amount rule, a fixed explore-count would be
//! a materially wrong card, so the whole effect is GAP'd.
//! ("Explore" is the Scryfall keyword shorthand for this ETB ability,
//! not a standalone keyword-line ability, so `keywords` is empty.)

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
    let name = reg.interner_mut().intern("Jadelight Spelunker");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let scout = reg.interner_mut().intern("Scout");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(scout);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: explore_x_times,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn explore_x_times(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "explores X times" — X is the cast x_value, which the
    // PendingTrigger surface does not expose; a fixed count would be
    // materially wrong.
    Vec::new()
}
