//! Volatile Rig — `{4}` 4/4 Artifact Creature — Construct with Trample.
//! Attacks each combat if able (GAP — static combat requirement).
//! When dealt damage, flip a coin; if you lose, sacrifice this (GAP — no
//!   immediate self-sacrifice primitive; the coin flip is retained).
//! When this dies, flip a coin; if you lose, deal 4 damage to each creature
//!   and each player.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Volatile Rig");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Trample],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // GAP: static "This creature attacks each combat if able".
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfIsDealtDamage {
                    combat_only: false,
                },
                intervening_if: None,
                effect: dealt_damage_coin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: dies_coin,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn dealt_damage_coin(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::FlipCoin {
        player: trig.controller,
        win: Box::new(Effect::Sequence(vec![])),
        // GAP: "sacrifice this creature" on a lost flip — no immediate
        // self-sacrifice effect in the demonstrated catalog.
        lose: Some(Box::new(Effect::Sequence(vec![]))),
    }]
}

fn dies_coin(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut wrath = Vec::new();
    let creatures = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    if !creatures.is_empty() {
        wrath.push(Effect::ForEach {
            targets: creatures,
            effect: Box::new(Effect::DealDamage {
                source: trig.source,
                target: DamageTarget::Object(NULL_OBJECT_ID),
                amount: 4,
            }),
        });
    }
    for p in script::all_players(state) {
        wrath.push(Effect::DealDamage {
            source: trig.source,
            target: DamageTarget::Player(p),
            amount: 4,
        });
    }
    vec![Effect::FlipCoin {
        player: trig.controller,
        win: Box::new(Effect::Sequence(vec![])),
        lose: Some(Box::new(Effect::Sequence(wrath))),
    }]
}
