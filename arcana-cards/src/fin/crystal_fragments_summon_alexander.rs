//! Crystal Fragments // Summon: Alexander — {W} Artifact — Equipment (front) /
//! Enchantment Creature — Saga Construct (back). Transform TDFC.
//!
//! Front face (Crystal Fragments):
//!   Equipped creature gets +1/+1.
//!   {5}{W}{W}: Exile this Equipment, then return it to the battlefield
//!              transformed under its owner's control. Activate only as a sorcery.
//!   Equip {1}
//!
//! Back face (Summon: Alexander) — Enchantment Creature — Saga Construct, Flying:
//!   (As this Saga enters and after your draw step, add a lore counter.
//!    Sacrifice after III.)
//!   I, II — Prevent all damage that would be dealt to creatures you control this turn.
//!   III   — Tap all creatures your opponents control.
//!
//! # Notes / GAPs
//! - The "equipped creature gets +1/+1" static is installed from a front-face
//!   ETB trigger via ContinuousEffect::attached_pt (the with_equip helper does
//!   not install the bonus itself).
//! - "{5}{W}{W}: Exile this Equipment, then return it transformed" is modeled
//!   as a sorcery-speed Transform activation (same posture as Elesh Norn:
//!   exile-and-return-transformed ≈ Transform).
//! - The back-face Saga chapters are authored as triggered abilities on the
//!   CardDefinition. The engine does not support face-gated triggered abilities,
//!   so they are active regardless of face; this is the established transform-
//!   Saga modeling (see Elesh Norn // The Argent Etchings).
//! - Back face's Flying / Saga Construct subtypes are emitted on the back face
//!   characteristics. The Saga's lore-counter progression after a transform is
//!   itself engine debt (enters-with applies to the front face); best-effort
//!   chapter authoring is provided.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardFace, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
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
use arcana_core::effects::KeywordAbility;
use arcana_core::objects::NULL_OBJECT_ID;
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crystal Fragments");
    let equipment_sub = reg.interner_mut().intern("Equipment");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(equipment_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ARTIFACT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    // Back face: Summon: Alexander — Enchantment Creature — Saga Construct, Flying.
    let back_name = reg.interner_mut().intern("Summon: Alexander");
    let saga_sub = reg.interner_mut().intern("Saga");
    let construct_sub = reg.interner_mut().intern("Construct");
    let mut back_subtypes = SubtypeSet::default();
    back_subtypes.0.insert(saga_sub);
    back_subtypes.0.insert(construct_sub);
    let back = CardFace {
        name: back_name,
        characteristics: Characteristics {
            name: back_name,
            colors: ColorSet::white(),
            types: TypeLine(TypeLine::ENCHANTMENT | TypeLine::CREATURE),
            subtypes: back_subtypes,
            keywords: vec![KeywordAbility::Flying],
            ..Default::default()
        },
        spell_ability: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_transform_back(back)
            .with_equip(ManaCost::parse("{1}").expect("valid cost"))
            // Front face: install the "equipped creature gets +1/+1" static on ETB.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_equip_bonus,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {5}{W}{W}: Exile this Equipment, return transformed. Sorcery speed.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{5}{W}{W}: Exile this Equipment, then return it to the battlefield transformed under its owner's control. Activate only as a sorcery.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{5}{W}{W}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: transform_activation,
            })
            // Back face Saga: add a lore counter on your first main phase.
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
            .with_triggered_ability(TriggeredAbilityDef {
                id: 3,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(1),
                },
                intervening_if: None,
                effect: chapter_i_ii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 4,
                trigger_condition: TriggerCondition::CounterAdded {
                    on: TriggerSelf::Source,
                    kind: Some(CounterKind::Lore),
                    chapter: Some(2),
                },
                intervening_if: None,
                effect: chapter_i_ii,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 5,
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

fn install_equip_bonus(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::attached_pt(
            trig.source,
            1,
            1,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
}

fn transform_activation(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Transform { target: ctx.source }]
}

fn add_lore_counter(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: trig.source,
        kind: CounterKind::Lore,
        count: 1,
    }]
}

/// Chapters I and II both prevent all damage that would be dealt to
/// creatures you control this turn.
fn chapter_i_ii(_state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::PreventDamageFrom {
        source_filter: ObjectFilter::permanent(),
        target_filter: TargetFilter::Permanent(
            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
        ),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}

/// Chapter III — Tap all creatures your opponents control.
fn chapter_iii(state: &GameState, trig: &PendingTrigger, _reg: &CardRegistry) -> Vec<Effect> {
    let filter = ObjectFilter::creature().controlled_by(ControllerConstraint::Opponent);
    let ids = script::ids_matching(state, &filter, trig.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Tap { target: NULL_OBJECT_ID }),
    }]
}
