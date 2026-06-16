//! Ziatora's Envoy — `{1}{B}{R}{G}` 5/4 Lizard Warrior.
//! Trample. (Blitz {2}{B}{R}{G} — not in the usable keyword surface, GAP.)
//! Whenever this creature deals combat damage to a player, look at the top
//! card of your library; you may play a land or cast a spell with mana value
//! <= the damage dealt from the top without paying its cost, else put it into
//! your hand.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::TargetFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ziatora's Envoy");
    let lizard = reg.interner_mut().intern("Lizard");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(lizard);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: Blitz {2}{B}{R}{G} — alternative cast cost not in the usable
    // keyword surface.

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: arcana_core::targets::ObjectFilter::permanent(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_dig(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at the top card; you may play a land or cast a spell with mv
    // <= damage from the top without paying its cost, else put it into your
    // hand" — a conditional, damage-gated top-of-library play/cast with a
    // hand fallback that no documented Effect expresses (ImpulseExile lacks
    // the mv gate and the to-hand fallback).
    Vec::new()
}
