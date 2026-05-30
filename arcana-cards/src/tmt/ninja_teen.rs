//! Ninja Teen — `{2}{B}` black Enchantment — Class.
//!
//! Level 1 (base): Whenever a creature you control leaves the battlefield,
//!   each opponent loses 1 life.
//! Level 2 ({1}{B}): Creatures you control get +1/+0 and have menace.
//! Level 3 ({B}): Creature cards in your graveyard have sneak {3}{B}.
//!   You may cast creature spells from your graveyard using their sneak abilities.
//!
//! GAP: Level 3 "sneak" mechanic — casting creature spells from graveyard via sneak
//!   is not in the Effect catalog (continuous-effect engine subsystem).

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Ninja Teen");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
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
            // Level 1: Whenever a creature you control leaves the battlefield,
            // each opponent loses 1 life.
            // GAP: ZoneChange.to is required; "leaves battlefield to any zone" is not
            // expressible — wired as dies-to-graveyard only; bounce/exile cases missed.
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::You),
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: creature_leaves_trigger,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            // Level 2: {1}{B} — creatures you control get +1/+0 and have menace.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{B}: Level 2".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{B}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_2,
            })
            // Level 3: {B} — creature cards in graveyard have sneak {3}{B} (GAP).
            .with_activated_ability(ActivatedAbilityDef {
                text: "{B}: Level 3".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{B}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_3,
            }),
    )
}

fn creature_leaves_trigger(
    state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let opponents = arcana_core::script::opponents(state, trig.controller);
    opponents.into_iter().map(|p| Effect::LoseLife { player: p, amount: 1 }).collect()
}

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
            effect: ContinuousEffect::anthem(
                ctx.source,
                ctx.controller,
                1,
                0,
                Duration::WhileSourceOnBattlefield,
            ),
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

fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Level 3 "creature cards in your graveyard have sneak {3}{B}" and
    // "you may cast creature spells from your graveyard using their sneak abilities"
    // — sneak mechanic is not in the Effect catalog (continuous-effect engine subsystem).
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}
