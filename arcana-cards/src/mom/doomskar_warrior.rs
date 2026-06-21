//! Doomskar Warrior — `{2}{G}{G}` 4/3 Human Warrior.
//! Backup 1.
//! Trample.
//! Whenever this creature deals combat damage to a player or battle, look at
//! that many cards from the top of your library. You may reveal a creature
//! or land card from among them and put it into your hand. Put the rest on
//! the bottom of your library in a random order.
//!
//! Trample is a base keyword. Backup 1 is not an emittable KeywordAbility
//! (its attach-the-counter-and-share-abilities ETB is unmodeled) — GAP'd.
//! The combat-damage trigger uses DigTopN with count = damage dealt
//! (trig.damage_amount), taking a creature-or-land card. The "or battle"
//! half is covered only for the player target (battles are unmodeled).

use arcana_core::effects::{DigRest, Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Doomskar Warrior");
    let human = reg.interner_mut().intern("Human");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(warrior);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    // GAP: Backup 1 — not an emittable KeywordAbility; counter-and-ability-
    // sharing ETB unmodeled.
    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::DamageDealt {
                source_filter: ObjectFilter::default(),
                target_filter: TargetFilter::Player,
                combat_only: true,
            },
            intervening_if: None,
            effect: combat_damage_dig,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: Vec::new(),
        }),
    )
}

fn combat_damage_dig(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = trig.damage_amount().unwrap_or(0);
    if n == 0 {
        return Vec::new();
    }
    vec![Effect::DigTopN {
        player: trig.controller,
        count: n,
        filter: Some(ObjectFilter::new().with_types_any(TypeLine(
            TypeLine::CREATURE | TypeLine::LAND,
        ))),
        rest: DigRest::BottomRandom,
    }]
}
