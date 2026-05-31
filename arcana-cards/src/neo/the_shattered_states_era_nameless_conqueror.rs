//! The Shattered States Era // Nameless Conqueror — transforming Saga // creature DFC.
//!
//! Front (The Shattered States Era): {4}{R} Enchantment — Saga.
//!   I  — Gain control of target creature until end of turn. Untap it. It gains haste
//!        until end of turn.
//!   II — Creatures you control get +1/+0 until end of turn.
//!   III — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back (Nameless Conqueror): Enchantment Creature — Human Samurai 4/4 with Trample, Haste.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Shattered States Era");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Nameless Conqueror");
    let human_sub = reg.interner_mut().intern("Human");
    let samurai_sub = reg.interner_mut().intern("Samurai");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub);
    back_subtypes.0.insert(samurai_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::red(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Trample, KeywordAbility::Haste],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Lore,
                count: 1,
            })
            // First main phase: add a lore counter.
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
            // I — Gain control of target creature until end of turn; untap; gains haste.
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
                target_requirements: vec![TargetRequirement::target_creature()],
            })
            // II — Creatures you control get +1/+0 until end of turn.
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
            // III — Exile this Saga, then return it transformed under your control.
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
            })
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = trig.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::ChangeControlEot {
            target: *id,
            new_controller: trig.controller,
        },
        Effect::Untap { target: *id },
        Effect::GrantKeyword {
            target: *id,
            keyword: KeywordAbility::Haste,
            duration: Duration::EndOfTurn,
        },
    ]
}

fn chapter_ii(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &filter, trig.controller);
    ids.into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: 1,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect()
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Exile this Saga, then return it transformed under your control.
    vec![
        Effect::ExilePermanent { target: trig.source },
        Effect::ReturnFromExileToBattlefield { target: trig.source },
        Effect::Transform { target: trig.source },
    ]
}
