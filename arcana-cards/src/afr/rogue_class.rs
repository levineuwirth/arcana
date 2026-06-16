//! Rogue Class — `{U}{B}` blue/black Enchantment — Class.
//! Level 1: "Whenever a creature you control deals combat damage to a player,
//!           exile the top card of that player's library face down. You may
//!           look at it for as long as it remains exiled."
//! Level 2: "Creatures you control have menace."
//! Level 3: "You may play cards exiled with this Class."
//! GAP: Level 1 "exile top card of that player's library face down and
//!      remember it" not expressible (Mill goes to GY, no face-down-exile).
//! GAP: Level 3 "play exiled cards" not expressible.
//!
//! Level 2's "Creatures you control have menace" is modeled as a
//! controller-wide keyword anthem installed when the Level-2 activation
//! resolves (Duration::WhileSourceOnBattlefield). Class levels never
//! decrease, so install-on-level-up is equivalent to a level-gated
//! static — no separate level-query is needed.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rogue Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            // Level 1: combat damage trigger
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::DamageDealt {
                    source_filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    target_filter: TargetFilter::Player,
                    combat_only: true,
                },
                intervening_if: None,
                effect: exile_top_card,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}{B}: Level 2".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}{B}").unwrap(),
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
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}{B}: Level 3".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}{B}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up,
            }),
    )
}

fn exile_top_card(
    _state: &GameState,
    _trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "exile top card of that player's library face down" not expressible;
    // Mill goes to graveyard not exile, and no face-down tracking
    Vec::new()
}

fn level_up(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}

/// Level 2: add the Level counter and install the "Creatures you
/// control have menace" controller-wide keyword anthem. The anthem
/// persists while this Class is on the battlefield; since Class levels
/// never decrease, that's equivalent to a static gated on level >= 2.
fn level_up_to_2(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Level,
            count: 1,
        },
        Effect::InstallContinuousEffect {
            effect: ContinuousEffect::keyword_anthem(
                ctx.source,
                ctx.controller,
                KeywordAbility::Menace,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
