//! The Long Reach of Night // Animus of Night's Reach — {3}{B} Enchantment — Saga
//! (transforming DFC saga; back face is a creature).
//!
//! (As this Saga enters and after your draw step, add a lore counter.)
//! I, II — Each opponent sacrifices a creature of their choice unless they discard a card.
//! III — Exile this Saga, then return it to the battlefield transformed under your control.
//! ---
//! Animus of Night's Reach — Enchantment Creature — Spirit, with Menace.
//!   Whenever this creature attacks, it gets +X/+0 until end of turn, where X is the number
//!   of creature cards in defending player's graveyard.
//!
//! Chapters I/II: "Each opponent sacrifices a creature of their choice" is wired as one
//! `Effect::ChooseNFromZone` per opponent (chooser = that opponent, action Sacrifice).
//! GAP (chapters I/II): "…unless they discard a card" — the per-opponent option to discard
//! instead of sacrificing is not expressible (OptionalPaymentKind only models Mana / Life,
//! no Discard gate); the sacrifice is wired as mandatory.
//!
//! GAP (chapter III): "Exile this Saga, then return it transformed under your control" — the
//! engine has Effect::Transform (an in-place flip) but no exile-and-return-transformed
//! primitive. Best-effort: Effect::Transform { target } flips the Saga to its creature back
//! face in place (the SBA final-chapter sacrifice would otherwise fire, so this chapter's flip
//! is authored as the chapter-III payload).
//!
//! GAP (back face): "it gets +X/+0 ... where X is the number of creature cards in defending
//! player's graveyard" — the defending player is not knowable from the attack trigger's
//! available accessors here (no defender accessor on PendingTrigger), so the dynamic pump is
//! emitted as Vec::new() rather than a wrong literal.

use arcana_core::effects::{Effect, KeywordAbility, PickAction};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardFace, CardRegistry, EntersWithSpec};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("The Long Reach of Night");
    let saga_sub = reg.interner_mut().intern("Saga");
    let spirit = reg.interner_mut().intern("Spirit");

    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    let back_name = reg.interner_mut().intern("Animus of Night's Reach");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(spirit);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(3)),
            toughness: Some(PtValue::Fixed(3)),
            keywords: vec![KeywordAbility::Menace],
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
            // Chapter I — each opponent sacrifices a creature unless they discard a card.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(1),
                },
                intervening_if: None,
                effect: chapter_sac_or_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Chapter II — same as chapter I.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: chapter_sac_or_discard,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Chapter III — exile this Saga, then return it transformed (best-effort: flip).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(3),
                },
                intervening_if: None,
                effect: chapter_transform,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Back-face: whenever this creature attacks, it gets +X/+0 (X = creature cards
            // in defending player's graveyard). See GAP note — emitted as no-op.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
                trigger_condition: TriggerCondition::SelfAttacks,
                intervening_if: None,
                effect: attack_pump,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
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

fn chapter_sac_or_discard(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Each opponent sacrifices a creature of their choice" — one
    // ChooseNFromZone per opponent; the controller constraint is evaluated
    // from the CHOOSER's perspective. Separate top-level effects so each
    // pending choice parks correctly.
    // GAP: "…unless they discard a card" — the discard-instead option is
    // not expressible; the sacrifice is wired as mandatory.
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::You);
    script::opponents(state, trig.controller)
        .into_iter()
        .map(|opp| Effect::ChooseNFromZone {
            chooser: opp,
            zone: Zone::Battlefield,
            filter: filter.clone(),
            min: 1,
            max: 1,
            action: PickAction::Sacrifice,
        })
        .collect()
}

fn chapter_transform(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Exile this Saga, then return it transformed under your control" — no
    // exile-and-return-transformed primitive; best-effort in-place flip to the back face.
    vec![Effect::Transform {
        target: trig.source,
    }]
}

fn attack_pump(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: X is the number of creature cards in DEFENDING PLAYER's graveyard, but the
    // defending player is not available from the attack trigger's accessors here.
    Vec::new()
}
