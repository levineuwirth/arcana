//! Urza's Saga
//! Enchantment Land — Urza's Saga (no mana cost)
//! I — This Saga gains "{T}: Add {C}."
//! II — This Saga gains "{2}, {T}: Create a 0/0 colorless Construct artifact creature token with
//!      'This token gets +1/+1 for each artifact you control.'"
//! III — Search your library for an artifact card with mana cost {0} or {1}, put it onto the battlefield, then shuffle.
//! GAP: Chapter I — "This Saga gains '{T}: Add {C}'" grants an activated mana ability to self;
//!      no GrantActivatedAbility effect in the catalog.
//! GAP: Chapter II — The chapter itself says "This Saga gains '{2}, {T}: Create a token'";
//!      the granted activated ability is not modelable. We approximate by creating the token directly.
//! GAP: Chapter II — Construct token's static "+1/+1 for each artifact you control" not modeled.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urza's Saga");
    let saga_sub = reg.interner_mut().intern("Urza's Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    // Pre-intern the Construct subtype so it's available at resolve time
    let _construct_sub = reg.interner_mut().intern("Construct");

    let chars = Characteristics {
        name,
        mana_cost: None,
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::LAND),
        subtypes,
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
                target_requirements: Vec::new(),
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
                target_requirements: Vec::new(),
            }),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "This Saga gains '{T}: Add {C}'" grants an activated mana ability to self.
    //      There is no GrantActivatedAbility effect in the catalog.
    Vec::new()
}

fn chapter_ii(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // GAP: The chapter text says "This Saga gains '{2}, {T}: Create a 0/0 Construct token'";
    //      the granted activated ability is not modelable. Approximating as direct token creation.
    // GAP: Construct token's static "+1/+1 for each artifact you control" not modeled.
    let construct_sub = reg.interner().lookup("Construct")
        .expect("Construct interned during register()");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(construct_sub);
    vec![Effect::CreateToken {
        controller: trig.controller,
        token: TokenDefinition {
            name: construct_sub,
            colors: ColorSet::colorless(),
            types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(0)),
            toughness: Some(PtValue::Fixed(0)),
            keywords: vec![],
            abilities: vec![],
        },
    }]
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::new()
        .with_types(TypeLine::ARTIFACT.into())
        .with_max_cmc(1);
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter,
        tapped: false,
    }]
}
