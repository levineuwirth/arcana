//! Kardur's Vicious Return — `{2}{B}{R}` black/red Enchantment — Saga.
//! I — You may sacrifice a creature. When you do, this Saga deals 3 damage to any target.
//! II — Each player discards a card.
//! III — Return target creature card from your graveyard to the battlefield. Put a +1/+1 counter on it. It gains haste until your next turn.
//! GAP: Chapter I "you may sacrifice a creature. When you do, deal 3 damage" — OptionalPayment with sacrifice cost not in catalog.
//! GAP: Chapter III — the return re-ids the object, so the follow-up counter and haste grant on the graveyard id may miss (no return-with-riders variant).
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kardur's Vicious Return");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Lore,
                count: 1,
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::PhaseBegins {
                    phase: Phase::PreCombatMain,
                    whose: ControllerConstraint::You,
                },
                intervening_if: None,
                effect: add_lore_counter,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(1),
                },
                intervening_if: None,
                effect: chapter_i,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: chapter_ii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(3),
                },
                intervening_if: None,
                effect: chapter_iii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature(),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
            }),
    )
}

fn add_lore_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters { target: trig.source, kind: CounterKind::Lore, count: 1 }]
}

fn chapter_i(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may sacrifice a creature. When you do, deal 3 damage" — sacrifice-conditional not in catalog
    // Emitting direct damage as approximation
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    match target {
        arcana_core::targets::TargetChoice::Object(id) => {
            vec![Effect::DealDamage { target: DamageTarget::Object(*id), amount: 3, source: trig.source }]
        }
        arcana_core::targets::TargetChoice::Player(p) => {
            vec![Effect::DealDamage { target: DamageTarget::Player(*p), amount: 3, source: trig.source }]
        }
        _ => Vec::new(),
    }
}

fn chapter_ii(
    state: &GameState,
    _trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let all_players = script::all_players(state);
    all_players.into_iter()
        .map(|p| Effect::Discard { player: p, count: 1, choice: DiscardChoice::ControllerChooses })
        .collect()
}

fn chapter_iii(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: the return re-ids the object, so the counter and haste grant
    // on the graveyard id may miss (no return-with-riders variant).
    vec![
        Effect::ReturnFromGraveyardToBattlefield { target: *id },
        Effect::AddCounters { target: *id, kind: CounterKind::PlusOnePlusOne, count: 1 },
        Effect::GrantKeyword { target: *id, keyword: KeywordAbility::Haste, duration: Duration::UntilYourNextTurn(trig.controller) },
    ]
}
