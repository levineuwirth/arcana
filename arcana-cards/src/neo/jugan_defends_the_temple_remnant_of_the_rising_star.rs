//! Jugan Defends the Temple // Remnant of the Rising Star — {2}{G} green transforming
//! Saga (CR 716 / 712).
//!
//! Front face (Jugan Defends the Temple): Enchantment — Saga.
//!   (As this Saga enters and after your draw step, add a lore counter.)
//!   I — Create a 1/1 green Human Monk creature token with "{T}: Add {G}."
//!   II — Put a +1/+1 counter on each of up to two target creatures.
//!   III — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back face (Remnant of the Rising Star): Enchantment Creature — Dragon Spirit, 5/5, Flying.
//!   Whenever another creature you control enters, you may pay {X}. When you do, put X
//!     +1/+1 counters on that creature.
//!   As long as you control five or more modified creatures, this creature gets +5/+5 and
//!     has trample.
//!
//! # Notes / GAPs
//! - Chapter I token's "{T}: Add {G}" mana ability is not modeled (token activated mana
//!   abilities are not in the demonstrated surface) — the bare 1/1 green Human Monk token is
//!   created.
//! - Chapter III "Exile this Saga, then return it transformed" is modeled as a self-transform
//!   to the back face. The engine's final-chapter sacrifice SBA is the documented transform path.
//! - Back face P/T is the printed 5/5 (Scryfall).
//! - GAP: back-face "Whenever another creature you control enters, you may pay {X}. When you
//!   do, put X +1/+1 counters on that creature." — the {X} optional payment with the variable
//!   counter count and the reflexive "when you do" trigger is not expressible.
//! - GAP: back-face "As long as you control five or more modified creatures, this creature
//!   gets +5/+5 and has trample." — a conditional static keyed on the count of modified
//!   creatures is not expressible (no modified-creature predicate / conditional static layer).

use arcana_core::effects::{Effect, KeywordAbility, TokenDefinition};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jugan Defends the Temple");
    let saga_sub = reg.interner_mut().intern("Saga");
    let dragon = reg.interner_mut().intern("Dragon");
    let spirit = reg.interner_mut().intern("Spirit");
    // Pre-intern token subtypes for resolution.
    let _ = reg.interner_mut().intern("Human");
    let _ = reg.interner_mut().intern("Monk");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Remnant of the Rising Star — Enchantment Creature — Dragon Spirit, 5/5, Flying.
    let back_name = reg.interner_mut().intern("Remnant of the Rising Star");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(dragon);
    back_subtypes.0.insert(spirit);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::green(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(5)),
            toughness: Some(PtValue::Fixed(5)),
            keywords: vec![KeywordAbility::Flying],
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
            // Chapter I — Create a 1/1 green Human Monk token (with "{T}: Add {G}"; GAP).
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
            // Chapter II — Put a +1/+1 counter on each of up to two target creatures.
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
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
            })
            // Chapter III — Exile this Saga, return it transformed under your control.
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
    // 1/1 green Human Monk token. GAP: its "{T}: Add {G}" mana ability is not modeled.
    let human = reg.interner().lookup("Human").expect("interned at register");
    let monk = reg.interner().lookup("Monk").expect("interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(human);
    token_subtypes.0.insert(monk);
    let token = TokenDefinition {
        name: human,
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes: token_subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        keywords: vec![],
        abilities: vec![],
    };
    vec![Effect::CreateToken {
        controller: trig.controller,
        token,
    }]
}

fn chapter_ii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // Put a +1/+1 counter on each of up to two target creatures.
    let mut effects = Vec::new();
    for target in trig.targets.targets.iter() {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            });
        }
    }
    effects
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Exile this Saga, then return it transformed under your control" — self-transform.
    vec![Effect::Transform {
        target: trig.source,
    }]
}
