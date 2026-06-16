//! Cleric Class — `{W}` white Enchantment — Class.
//!
//! Level 1 (base):
//!   If you would gain life, you gain that much life plus 1 instead.
//! Level 2 ({3}{W}):
//!   Whenever you gain life, put a +1/+1 counter on target creature you control.
//! Level 3 ({4}{W}):
//!   When this Class becomes level 3, return target creature card from your
//!   graveyard to the battlefield. You gain life equal to that creature's toughness.
//!
//! GAP: Level 1 "if you would gain life, you gain that much life plus 1 instead"
//!      — replacement effect; not expressible as a P/T or keyword anthem.
//!      Deferred (continuous-effect engine subsystem).
//! GAP: Level 2 "Whenever you gain life, put a +1/+1 counter on target creature
//!      you control" — triggered ability based on GainLife events; not expressible
//!      as install-on-level-up continuous effect. Deferred.
//! The level-3 on-level-up trigger (return creature from graveyard, gain life
//! equal to its toughness) is a triggered ability that fires when the class
//! becomes level 3 — modeled via install-level-3 effect fn. However, the
//! "gain life equal to its toughness" is DYNAMIC and requires a script helper;
//! the return is modeled via ReturnFromGraveyardToBattlefield. The dynamic life
//! gain based on the returned creature's toughness is GAP'd (can't query the
//! toughness of the just-entered creature from the resolver context).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cleric Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let cleric_sub = reg.interner_mut().intern("Cleric");
    subtypes.0.insert(cleric_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
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
            // Level 2: {3}{W} — whenever you gain life, put +1/+1 counter on target creature.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{W}: Level 2".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{W}").unwrap(),
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
            // Level 3: {4}{W} — when this class becomes level 3, return a creature
            // from graveyard to battlefield, gain life equal to its toughness.
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{W}: Level 3".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{W}").unwrap(),
                    min_self_counters: Some((CounterKind::Level, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Card {
                        zone: Zone::Graveyard(0),
                        filter: ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    },
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: level_up_to_3,
            }),
    )
}

/// Level 2: bump level counter.
/// GAP: "Whenever you gain life, put a +1/+1 counter on target creature you control"
///      — GainLife event trigger not expressible as an install-on-level-up continuous effect.
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
        // GAP: per-level granted ability deferred (continuous-effect engine subsystem)
        // "Whenever you gain life, put a +1/+1 counter on target creature you control"
        // is not an anthem-style continuous effect; requires GainLife event trigger.
    ]
}

/// Level 3: bump level counter, return target creature from graveyard to battlefield.
/// GAP: "You gain life equal to that creature's toughness" — dynamic life gain
///      based on the just-entered creature's toughness not expressible.
fn level_up_to_3(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return vec![Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Level,
            count: 1,
        }];
    };
    let TargetChoice::Object(id) = target else {
        return vec![Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Level,
            count: 1,
        }];
    };
    let creature_id = *id;
    vec![
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::Level,
            count: 1,
        },
        Effect::ReturnFromGraveyardToBattlefield { target: creature_id },
        // GAP: "You gain life equal to that creature's toughness" — the toughness
        // of the just-returned creature is not accessible from the resolver context.
    ]
}
