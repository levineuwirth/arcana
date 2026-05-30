//! Fugitive of the Judoon — `{4}{G}` green Enchantment — Saga.
//!
//! I — Create a 1/1 white Human creature token with ward {2} and a 4/4 white Alien Rhino
//!     creature token.
//! II — Investigate.
//! III — You may exile a Human you control and an artifact you control. If you do, search your
//!       library for a Doctor card, put it onto the battlefield, then shuffle.
//!
//! # GAPs
//! - Chapter I: The 1/1 Human token has ward {2}. The engine wires ward on tokens via
//!   `keywords: vec![KeywordAbility::Ward(...)]`. The 4/4 Alien Rhino token has unusual subtypes
//!   "Alien" and "Rhino" which are valid to intern.
//! - Chapter III: "You may exile a Human you control and an artifact you control. If you do,"
//!   is a conditional optional-sacrifice gate — not expressible via OptionalPaymentKind.
//!   Best-effort: search for a Doctor card unconditionally.
//!   GAP: "you may exile a Human and artifact you control" cost gate not modeled.
//! - "Search for a Doctor card" — "Doctor" is a creature subtype; TutorToBattlefield with
//!   a subtype filter.

use arcana_core::effects::{CommodityToken, Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::script;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Fugitive of the Judoon");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    // Pre-intern token subtypes
    let _human_sub = reg.interner_mut().intern("Human");
    let _alien_sub = reg.interner_mut().intern("Alien");
    let _rhino_sub = reg.interner_mut().intern("Rhino");
    let _doctor_sub = reg.interner_mut().intern("Doctor");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
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

fn chapter_i(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // 1/1 white Human creature token with ward {2}
    let human_id = reg.interner().lookup("Human").expect("Human interned");
    let mut human_subtypes = SubtypeSet::default();
    human_subtypes.0.insert(human_id);
    let human_token = TokenDefinition {
        name: human_id,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: human_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![KeywordAbility::Ward(ManaCost::parse("{2}").expect("ward cost"))],
        abilities: vec![],
    };

    // 4/4 white Alien Rhino creature token
    let alien_id = reg.interner().lookup("Alien").expect("Alien interned");
    let rhino_id = reg.interner().lookup("Rhino").expect("Rhino interned");
    let mut rhino_subtypes = SubtypeSet::default();
    rhino_subtypes.0.insert(alien_id);
    rhino_subtypes.0.insert(rhino_id);
    let rhino_token = TokenDefinition {
        name: rhino_id,
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes: rhino_subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        keywords: vec![],
        abilities: vec![],
    };

    vec![
        Effect::CreateToken { controller: trig.controller, token: human_token },
        Effect::CreateToken { controller: trig.controller, token: rhino_token },
    ]
}

fn chapter_ii(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Investigate — create a Clue token
    vec![Effect::CreateCommodityToken {
        controller: trig.controller,
        kind: CommodityToken::Clue,
        count: 1,
    }]
}

fn chapter_iii(
    _state: &GameState,
    trig: &PendingTrigger,
    reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may exile a Human you control and an artifact you control. If you do," gate
    // not expressible — searching library unconditionally.
    let doctor_filter = script::subtype_filter(reg, "Doctor");
    vec![Effect::TutorToBattlefield {
        player: trig.controller,
        filter: doctor_filter,
        tapped: false,
    }]
}
