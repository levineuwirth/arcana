//! Vault 11: Voter's Dilemma — {2}{W}{B} Enchantment — Saga
//! I: For each opponent, create a 1/1 white Human Soldier creature token.
//! II, III: Each player secretly votes for up to one creature; those votes revealed.
//!   If no creature got votes, each player draws a card.
//!   Otherwise, destroy each creature with the most votes or tied for most votes.
//! GAP: Vote mechanic (II, III) is not expressible with the current engine API.
//!   Chapters II and III emit no effect.

use arcana_core::effects::{Effect, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vault 11: Voter's Dilemma");
    let saga_sub = reg.interner_mut().intern("Saga");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    // Pre-intern token subtypes so lookup() succeeds in chapter_i at resolve time
    let _human_sub = reg.interner_mut().intern("Human");
    let _soldier_sub = reg.interner_mut().intern("Soldier");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        keywords: vec![],
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
                effect: chapter_ii_iii,
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
                effect: chapter_ii_iii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

fn add_lore_counter(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // For each opponent, create a 1/1 white Human Soldier creature token.
    let opponents = script::opponents(state, trig.controller);
    let human_id = reg.interner().lookup("Human");
    let soldier_id = reg.interner().lookup("Soldier");

    let mut token_subtypes = SubtypeSet::default();
    if let Some(h) = human_id {
        token_subtypes.0.insert(h);
    }
    if let Some(s) = soldier_id {
        token_subtypes.0.insert(s);
    }

    let name_id = soldier_id.unwrap_or(0);
    opponents
        .into_iter()
        .map(|_opp| Effect::CreateToken {
            controller: trig.controller,
            token: TokenDefinition {
                name: name_id,
                colors: ColorSet::white(),
                types: TypeLine::CREATURE.into(),
                subtypes: token_subtypes.clone(),
                power: Some(PtValue::Fixed(1)),
                toughness: Some(PtValue::Fixed(1)),
                keywords: vec![],
                abilities: vec![],
            },
        })
        .collect()
}

fn chapter_ii_iii(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Vote mechanic not expressible with current engine API.
    // "Each player secretly votes for up to one creature; if no creature got votes, each player
    // draws a card. Otherwise, destroy each creature with the most votes or tied for most votes."
    Vec::new()
}
