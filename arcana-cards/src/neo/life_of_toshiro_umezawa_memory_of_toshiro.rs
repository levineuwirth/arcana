//! Life of Toshiro Umezawa // Memory of Toshiro — `{1}{B}` Enchantment — Saga
//! that transforms into Memory of Toshiro (Enchantment Creature — Human Samurai).
//! (As this Saga enters and after your draw step, add a lore counter.)
//! I, II — Choose one —
//!   • Target creature gets +2/+2 until end of turn.
//!   • Target creature gets -1/-1 until end of turn.
//!   • You gain 2 life.
//! III — Exile this Saga, then return it to the battlefield transformed under your control.
//! Back (Memory of Toshiro):
//!   {T}, Pay 1 life: Add {B}. Spend this mana only to cast an instant or sorcery spell.
//!
//! GAPs:
//! - Chapters I and II are modal ("Choose one — …") triggered abilities. Triggered
//!   abilities cannot carry a modal choice in the engine (only spell abilities have
//!   `ModalSpec`), so the player can't pick a mode. Emitted as no-ops rather than
//!   forcing one wrong mode.
//! - Chapter III: "Exile this Saga, then return it to the battlefield transformed."
//!   Modeled with `Effect::Transform` (in-place face swap) — the exile-and-return is
//!   not separately expressible; this is the closest faithful approximation.
//! - Back face mana ability's "Spend this mana only to cast an instant or sorcery"
//!   restriction is not expressible (ManaUnit::plain is unrestricted); the {B} is added
//!   without the spend restriction.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationCost, ActivationContext, ActivationZone, CardDefinition,
    CardFace, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggerSelf, TriggeredAbilityDef,
};
use arcana_core::turn::Phase;
use arcana_core::types::{CardId, ColorSet, CounterKind, ManaColor, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Life of Toshiro Umezawa");
    let saga_sub = reg.interner_mut().intern("Saga");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(saga_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };

    // Back face: Memory of Toshiro — Enchantment Creature — Human Samurai.
    let back_name = reg.interner_mut().intern("Memory of Toshiro");
    let human_sub = reg.interner_mut().intern("Human");
    let samurai_sub = reg.interner_mut().intern("Samurai");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(human_sub);
    back_subtypes.0.insert(samurai_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::black(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            power: Some(PtValue::Fixed(2)),
            toughness: Some(PtValue::Fixed(2)),
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
            // Chapter I — modal (GAP: triggers can't carry modal choice).
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(1),
                },
                intervening_if: None,
                effect: chapter_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Chapter II — same modal.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: chapter_modal,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Chapter III — exile, then return transformed (modeled as in-place transform).
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
            // Chapters I/II live on the front face (the Saga).
            .with_trigger_face_gate(1, 0)
            .with_trigger_face_gate(2, 0)
            .with_trigger_face_gate(3, 0)
            .with_trigger_face_gate(4, 0)
            // Back face: {T}, Pay 1 life: Add {B}. (Spend restriction GAP.)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Pay 1 life: Add {B}. Spend this mana only to cast an instant \
                       or sorcery spell."
                    .into(),
                cost: ActivationCost {
                    tap: true,
                    life: 1,
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: true,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: Some(1),
                effect: add_black,
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

fn chapter_modal(_state: &GameState, _trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "Choose one — +2/+2 / -1/-1 / gain 2 life" is a modal choice on a
    // triggered ability, which the engine cannot present. No-op.
    Vec::new()
}

fn chapter_iii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    // "Exile this Saga, then return it transformed under your control."
    // Modeled as an in-place transform (closest expressible approximation).
    vec![Effect::Transform { target: trig.source }]
}

fn add_black(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddMana {
        player: ctx.controller,
        mana: vec![ManaUnit::plain(ManaColor::Black, ctx.source)],
    }]
}
