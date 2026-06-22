//! Chittering Skullspeaker — `{1}{B}` 1/1 Squirrel Warlock.
//!
//! Oracle:
//! * Intensity (Scryfall keyword) / "Starting intensity 0" — the Intensity
//!   mechanic is not in the usable keyword surface and has no state model;
//!   GAP'd.
//! * "When Chittering Skullspeaker enters, cards you own named Chittering
//!    Skullspeaker intensify by 1. Then you draw X cards and lose X life, where
//!    X is Chittering Skullspeaker's intensity." — GAP: "intensify" and a card's
//!    "intensity" value are unmodeled, so X is uncomputable; emitting a fixed
//!    draw/lose would be materially wrong.

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
    let name = reg.interner_mut().intern("Chittering Skullspeaker");
    let squirrel = reg.interner_mut().intern("Squirrel");
    let warlock = reg.interner_mut().intern("Warlock");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(squirrel);
    subtypes.0.insert(warlock);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        // GAP: Intensity / "Starting intensity 0" — no intensity state model.
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::SelfEntersBattlefield,
            intervening_if: None,
            effect: etb_intensify,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn etb_intensify(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "intensify by 1" and "X is this card's intensity" — the Intensity
    //      mechanic has no state model, so X is uncomputable.
    Vec::new()
}
