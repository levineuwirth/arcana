//! Caretaker's Talent — `{2}{W}` white Enchantment — Class.
//! Level 1 (base): Whenever one or more tokens you control enter, draw a card.
//!                 This ability triggers only once each turn.
//! {W}: Level 2 — When this Class becomes level 2, create a token that's a copy
//!               of target token you control.
//! {3}{W}: Level 3 — Creature tokens you control get +2/+2.
//! GAP: Level 1 "triggers only once each turn" frequency not expressible
//!      (TriggerFrequency::EachTime is used; once-per-turn gating not modeled).
//! GAP: Level 2 "when this Class becomes level 2, create a copy of target token"
//!      is a triggered ability on level-up; modeled as the level-2 activation effect
//!      creating a copy of target token you control. Targeting not directly supported
//!      on ActivatedAbilityDef in the same way; the copy effect is emitted but without
//!      a true target selection (GAP: CopyPermanent needs an ObjectId from the resolver context).
//!      Level 2 activation simply adds the counter; the "when you level up, copy a token" trigger is GAP.
//! GAP: Level 3 "Creature tokens you control get +2/+2" — anthem applied to all creatures
//!      you control (no tokens-only variant available in ContinuousEffect::anthem).

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Caretaker's Talent");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 1 base triggered ability: whenever one or more tokens you control enter,
            // draw a card. GAP: once-per-turn gating not modeled.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::new()
                        .tokens_only()
                        .controlled_by(ControllerConstraint::You),
                    from: None,
                    to: Zone::Battlefield,
                },
                intervening_if: None,
                effect: draw_on_token_etb,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // {W}: Level 2 — requires level >= 1
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}: Level 2".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_2,
            })
            // {3}{W}: Level 3 — requires level >= 2
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W}: Level 3".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_3,
            }),
    )
}

fn draw_on_token_etb(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: trig.controller, count: 1 }]
}

fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Add Level counter. GAP: "create a token that's a copy of target token you control"
    // is a triggered effect on becoming level 2 that requires a target selection;
    // not expressible as part of the level-up activation effect (no target access here).
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}

fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // Add Level counter and install "Creature tokens you control get +2/+2" anthem.
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Level,
            count: 1,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::anthem(
                ctx.source,
                ctx.controller,
                2,
                2,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
