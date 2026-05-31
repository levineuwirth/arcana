//! The Rise of Sozin // Fire Lord Sozin — `{4}{B}{B}` Enchantment — Saga.
//!
//! Front face (The Rise of Sozin — Enchantment — Saga):
//! (As this Saga enters and after your draw step, add a lore counter.)
//! I — Destroy all creatures.
//! II — Choose a card name. Search target opponent's graveyard, hand, and library for up to
//!   four cards with that name and exile them. Then that player shuffles.
//! III — Exile this Saga, then return it to the battlefield transformed under your control.
//!
//! Back face (Fire Lord Sozin — Legendary Creature — Human Noble, 4/4):
//! Menace, firebending 3 (Whenever this creature attacks, add {R}{R}{R}.)
//! Whenever Fire Lord Sozin deals combat damage to a player, you may pay {X}. When you do,
//!   put any number of target creature cards with total mana value X or less from that
//!   player's graveyard onto the battlefield under your control.
//!
//! GAP: Chapter II ("choose a card name, search opponent's graveyard/hand/library for up to
//!   four cards with that name and exile them, then shuffle") — name-choice plus cross-zone
//!   multi-card exile is not expressible with the available effect surface. Emitted as a no-op.
//! GAP: Chapter III "exile this Saga, then return it transformed" — modeled as in-place
//!   `Effect::Transform`; the exile-and-return blink is not separately expressible.
//! GAP: back-face combat-damage trigger ("pay {X}, then reanimate any number of target creature
//!   cards with total mana value X or less from that player's graveyard") — variable {X}
//!   payment driving a total-mana-value-bounded multi-target reanimation is not expressible;
//!   trigger is declared but its body is a no-op.
//! GAP: "firebending 3" is not in the modeled keyword list; modeled as a back-face SelfAttacks
//!   trigger that adds {R}{R}{R} (the "until end of combat" duration on the mana is not modeled).

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Rise of Sozin");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    // Back face — Fire Lord Sozin (Legendary Creature — Human Noble, 4/4)
    let back_name = reg.interner_mut().intern("Fire Lord Sozin");
    let human_sub = reg.interner_mut().intern("Human");
    let noble_sub = reg.interner_mut().intern("Noble");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub);
    back_subtypes.0.insert(noble_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine::CREATURE.into(),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            power: Some(PtValue::Fixed(4)),
            toughness: Some(PtValue::Fixed(4)),
            keywords: vec![KeywordAbility::Menace],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Lore,
                count: 1,
            })
            .with_transform_back(back)
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
            // Chapter I — Destroy all creatures.
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
            // Chapter II — GAP (choose-name cross-zone exile not expressible).
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
            // Chapter III — Exile this Saga, then return it transformed.
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
            // Back face — firebending 3: whenever Fire Lord Sozin attacks, add {R}{R}{R}.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: firebending,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back face — combat damage to a player: pay {X}, reanimate (GAP body).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 6,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::new(),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: combat_damage_reanimate,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(5, 1)
            .with_trigger_face_gate(6, 1),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

fn chapter_i(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let ids = script::ids_matching(state, &ObjectFilter::creature(), trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent {
            target: NULL_OBJECT_ID,
        }),
    }]
}

fn chapter_ii(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Choose a card name. Search target opponent's graveyard, hand, and library for up
    // to four cards with that name and exile them. Then that player shuffles." — name choice
    // plus cross-zone multi-card exile is not expressible.
    Vec::new()
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "exile, then return transformed" — modeled as in-place Transform.
    vec![Effect::Transform { target: trig.source }]
}

fn firebending(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "this mana lasts until end of combat" duration not modeled.
    vec![Effect::AddMana {
        player: trig.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, trig.source); 3],
    }]
}

fn combat_damage_reanimate(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you may pay {X}. When you do, put any number of target creature cards with total
    // mana value X or less from that player's graveyard onto the battlefield under your
    // control." — variable {X} payment driving total-mana-value-bounded multi-target
    // reanimation is not expressible.
    Vec::new()
}
