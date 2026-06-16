//! Summon: Good King Mog XII — `{4}{W}` white Enchantment Creature — Saga Moogle.
//! Flying, lifelink. 4/4. 4 chapters.
//! I — Create two 1/2 white Moogle creature tokens with lifelink.
//! II, III — Whenever you cast a noncreature spell this turn, create a token copy of a non-Saga token you control.
//! IV — Put two +1/+1 counters on each other Moogle you control.
//! Chapter II/III wired via `Effect::EachCastThisTurn { Noncreature, CopyANonSagaTokenYouControl }`
//! (the copied token is the deterministic lowest-id pick — delayed triggers carry no choices).
//! Final-chapter sacrifice is automatic (engine SBA).

use arcana_core::effects::{EachCastRider, Effect, KeywordAbility, NextCastKind, TokenDefinition};
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
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Summon: Good King Mog XII");
    let saga_sub = reg.interner_mut().intern("Saga");
    let moogle_sub = reg.interner_mut().intern("Moogle");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);
    subtypes.0.insert(moogle_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet::default(),
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::Lifelink],
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
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(4),
                },
                intervening_if: None,
                effect: chapter_iv,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
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
    reg: &CardRegistry,
) -> Vec<Effect> {
    let moogle = reg.interner().lookup("Moogle").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(moogle);
    let token = TokenDefinition {
        name: moogle,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Lifelink],
        abilities: vec![],
    };
    vec![
        Effect::CreateToken { controller: trig.controller, token: token.clone() },
        Effect::CreateToken { controller: trig.controller, token },
    ]
}

fn chapter_ii_iii(
    _state: &GameState,
    trig: &PendingTrigger,
    _: &CardRegistry,
) -> Vec<Effect> {
    // "Whenever you cast a noncreature spell this turn, create a token
    // copy of a non-Saga token you control" — repeating, lapses at end
    // of turn. The copied token is the engine's deterministic pick
    // (lowest-id non-Saga token you control).
    vec![Effect::EachCastThisTurn {
        controller: trig.controller,
        kind: NextCastKind::Noncreature,
        rider: EachCastRider::CopyANonSagaTokenYouControl,
    }]
}

fn chapter_iv(
    state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let moogle_filter = script::subtype_filter(reg, "Moogle");
    let ids = script::ids_matching(state, &moogle_filter, trig.controller);
    ids.into_iter()
        .filter(|id| *id != trig.source)
        .flat_map(|id| {
            vec![
                Effect::AddCounters { target: id, kind: CounterKind::PlusOnePlusOne, count: 1 },
                Effect::AddCounters { target: id, kind: CounterKind::PlusOnePlusOne, count: 1 },
            ]
        })
        .collect()
}
