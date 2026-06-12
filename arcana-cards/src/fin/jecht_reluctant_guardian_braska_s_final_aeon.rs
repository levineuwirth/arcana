//! Jecht, Reluctant Guardian // Braska's Final Aeon
//!
//! Front face: {3}{B} Legendary Creature — Human Warrior 4/3
//! Menace
//! Whenever Jecht deals combat damage to a player, you may exile it, then return it to the
//! battlefield transformed under its owner's control.
//!
//! Back face: Legendary Enchantment Creature — Saga Nightmare
//! (As this Saga enters and after your draw step, add a lore counter. Sacrifice after III.)
//! I, II — Jecht Beam — Each opponent discards a card and you draw a card.
//! III — Ultimate Jecht Shot — Each opponent sacrifices two creatures of their choice.
//! Menace
//!
//! Back-face Saga chapters wired as face-gated triggered abilities (CounterAdded chapter
//! triggers + a face-gated lore-counter trigger, mirroring The Long Reach of Night):
//! I/II — each opponent discards a card and you draw a card; III — each opponent
//! sacrifices two creatures of their choice via `Effect::ChooseNFromZone` per opponent.
//! GAP: "As this Saga enters … add a lore counter" — the transform is an in-place flip
//!      (no re-entry), so no enters-with lore counter; chapter I fires at the next
//!      precombat main (the "after your draw step" lore-add approximation).
//! GAP: Transform trigger condition is "whenever Jecht deals combat damage to a player" —
//!      approximated via TriggerCondition::DamageDealt with combat_only: true.
//! GAP: "you may exile it, then return it transformed" — exile+return-transformed not a
//!      single Effect variant; approximated as Effect::Transform.
//! GAP: back face P/T not given in card spec (Saga+Creature hybrid — left unset).

use arcana_core::effects::{DiscardChoice, Effect, KeywordAbility, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jecht, Reluctant Guardian");
    let human_sub = reg.interner_mut().intern("Human");
    let warrior_sub = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(human_sub);
    subtypes.0.insert(warrior_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        keywords: vec![KeywordAbility::Menace],
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // Back face: Braska's Final Aeon — Legendary Enchantment Creature — Saga Nightmare
    let back_name = reg.interner_mut().intern("Braska's Final Aeon");
    let saga_sub = reg.interner_mut().intern("Saga");
    let nightmare_sub = reg.interner_mut().intern("Nightmare");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(saga_sub);
    back_subtypes.0.insert(nightmare_sub);

    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
            keywords: vec![KeywordAbility::Menace],
            // GAP: back face is a Saga+Creature hybrid; P/T not given in card spec.
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            // Front face: whenever Jecht deals combat damage to a player, transform.
            // Approximated as DamageDealt with combat_only: true targeting a player.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: jecht_damage_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![],
            })
            // Back face: lore-counter progression ("after your draw step,
            // add a lore counter" — approximated at precombat main, as in
            // The Long Reach of Night).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
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
            // I — Jecht Beam.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(1),
                },
                intervening_if: None,
                effect: jecht_beam,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // II — Jecht Beam (same as chapter I).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: jecht_beam,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // III — Ultimate Jecht Shot.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(3),
                },
                intervening_if: None,
                effect: ultimate_jecht_shot,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_trigger_face_gate(2, 1)
            .with_trigger_face_gate(3, 1)
            .with_trigger_face_gate(4, 1)
            .with_trigger_face_gate(5, 1),
    )
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

/// I, II — "Jecht Beam — Each opponent discards a card and you draw a card."
fn jecht_beam(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::Discard {
            player: opp,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        })
        .collect();
    effects.push(Effect::DrawCards {
        player: trig.controller,
        count: 1,
    });
    effects
}

/// III — "Ultimate Jecht Shot — Each opponent sacrifices two creatures of
/// their choice." One ChooseNFromZone per opponent; the controller
/// constraint is evaluated from the CHOOSER's perspective. Separate
/// top-level effects so each pending choice parks correctly.
fn ultimate_jecht_shot(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::ChooseNFromZone {
            chooser: opp,
            zone: Zone::Battlefield,
            filter: filter.clone(),
            min: 2,
            max: 2,
            action: PickAction::Sacrifice,
        })
        .collect()
}

fn jecht_damage_transform(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "You may exile it, then return it to the battlefield transformed."
    // GAP: exile+return-transformed not a single Effect; using Transform as approximation.
    // GAP: "you may" — optional choice not modeled; fires unconditionally.
    vec![Effect::Transform { target: trig.source }]
}
