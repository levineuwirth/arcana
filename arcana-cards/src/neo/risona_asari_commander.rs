//! Risona, Asari Commander — `{1}{R}{W}` 3/3 Legendary Human Samurai (R/W).
//! Haste.
//! Whenever Risona deals combat damage to a player, if it doesn't have an
//! indestructible counter on it, put an indestructible counter on it.
//! Whenever combat damage is dealt to you, remove an indestructible counter
//! from Risona.
//!
//! Haste is a base keyword. The first trigger is gated by an intervening-if
//! ("if it doesn't have an indestructible counter") and adds a Named
//! "indestructible" counter. The second fires on combat damage to a player and,
//! when that player is this creature's controller, removes one such counter.

use arcana_core::conditions;
use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PlayerId, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Risona, Asari Commander");
    let human = reg.interner_mut().intern("Human");
    let samurai = reg.interner_mut().intern("Samurai");
    let _ind = reg.interner_mut().intern("indestructible");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human);
    subtypes.0.insert(samurai);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Haste],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: Some(if_no_indestructible_counter),
                effect: add_indestructible_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: remove_indestructible_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn if_no_indestructible_counter(
    s: &GameState,
    src: ObjectId,
    _you: PlayerId,
    reg: &CardRegistry,
) -> bool {
    match reg.interner().lookup("indestructible").map(CounterKind::Named) {
        Some(kind) => !conditions::source_has_counter(s, src, kind),
        None => true,
    }
}

fn add_indestructible_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(kind) = reg.interner().lookup("indestructible").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: trig.source,
        kind,
        count: 1,
    }]
}

fn remove_indestructible_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // "Whenever combat damage is dealt to YOU" — only act when the damaged
    // player is this creature's controller.
    let Some(p) = trig.damaged_player() else {
        return Vec::new();
    };
    if p != trig.controller {
        return Vec::new();
    }
    let Some(kind) = reg.interner().lookup("indestructible").map(CounterKind::Named) else {
        return Vec::new();
    };
    vec![Effect::RemoveCounters {
        target: trig.source,
        kind,
        count: 1,
    }]
}
