//! The Bloodsky Massacre — `{1}{B}{R}` Enchantment — Saga.
//! (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
//! I — Create a 2/3 red Demon Berserker creature token with menace.
//! II — Whenever a Berserker attacks this turn, you draw a card and you lose 1 life.
//! III — Add {R} for each Berserker you control. Until end of turn, you don't lose
//!       this mana as steps and phases end.
//!
//! GAP (chapter II): "Whenever a Berserker attacks THIS TURN, ..." creates a
//!   delayed/this-turn-scoped triggered ability — not expressible with the
//!   demonstrated API (no delayed-triggered-ability effect). Emitted as a no-op.
//! GAP (chapter III rider): "Until end of turn, you don't lose this mana as steps
//!   and phases end." — mana that doesn't empty between steps is not modeled.

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef, TriggerSelf,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Bloodsky Massacre");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    // Pre-intern token subtypes for resolution.
    let _ = reg.interner_mut().intern("Demon");
    let _ = reg.interner_mut().intern("Berserker");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Lore,
                count: 1,
            })
            // "After your draw step, add a lore counter." (CR 716.3)
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
            // Chapter I: Create a 2/3 red Demon Berserker token with menace.
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
            // Chapter II: delayed this-turn Berserker-attack trigger.
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
            // Chapter III: Add {R} for each Berserker you control.
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

fn chapter_i(_state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let demon_sym = reg.interner().lookup("Demon").expect("interned at register");
    let berserker_sym = reg.interner().lookup("Berserker").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(demon_sym);
    token_subtypes.0.insert(berserker_sym);
    let token = TokenDefinition {
        name: demon_sym,
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Menace],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

fn chapter_ii(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Whenever a Berserker attacks this turn, you draw a card and you lose 1 life."
    // This is a delayed/this-turn-scoped triggered ability — not expressible with the
    // demonstrated API.
    Vec::new()
}

fn chapter_iii(state: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    // Add {R} for each Berserker you control.
    let berserker_filter = script::subtype_filter(reg, "Berserker")
        .controlled_by(ControllerConstraint::You);
    let count = script::count_matching(state, &berserker_filter, trig.controller) as usize;
    if count == 0 {
        return Vec::new();
    }
    // GAP: "Until end of turn, you don't lose this mana as steps and phases end."
    // — mana that doesn't empty is not modeled.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source); count],
    }]
}
