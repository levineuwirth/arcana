//! Trash Panda — `{4}{B}{G}` 5/5 Raccoon Bear.
//! When it enters, exile target opponent's graveyard and create a Food
//! token per creature card exiled this way. When it dies, put it into a
//! target opponent's graveyard. Opponent dredge 4.
//!
//! Keywords Food / Mill are Scryfall mechanic tags that are not part of
//! the usable `KeywordAbility` surface for this card class, so the
//! keyword line is empty. All three printed abilities are GAP'd: there
//! is no whole-graveyard-exile effect, no "count creature cards exiled
//! this way" accessor, no "put a dying object into another player's
//! graveyard" effect, and Dredge is not modeled.

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
    let name = reg.interner_mut().intern("Trash Panda");
    let raccoon = reg.interner_mut().intern("Raccoon");
    let bear = reg.interner_mut().intern("Bear");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(raccoon);
    subtypes.0.insert(bear);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: etb_exile_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_to_opponent_graveyard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn etb_exile_graveyard(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no effect exiles an entire player's graveyard, nor an accessor
    // to count creature cards exiled this way for the Food token rider.
    Vec::new()
}

fn dies_to_opponent_graveyard(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no effect places a dying object into another player's graveyard.
    Vec::new()
}

// GAP: "Opponent dredge 4" — Dredge is not modeled (no KeywordAbility::Dredge,
// no replacement for an opponent's draw with a mill-and-return).
