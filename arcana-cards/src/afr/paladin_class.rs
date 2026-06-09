//! Paladin Class — `{W}` white Enchantment — Class.
//!
//! Level 1 (base):
//!   Spells your opponents cast during your turn cost {1} more to cast.
//!
//! Level 2 ({2}{W}):
//!   Creatures you control get +1/+1.
//!
//! Level 3 ({4}{W}):
//!   Whenever you attack, until end of turn, target attacking creature gets
//!   +1/+1 for each other attacking creature and gains double strike.
//!
//! # GAPs
//! - Level 1 "opponents' spells cost {1} more during your turn" is a
//!   cost-modification continuous effect; not in ContinuousEffect builders.
//!   // GAP: Level 1 opponent spell cost increase — continuous-effect engine
//!   debt (cost modification not in ContinuousEffect builders).
//! - Level 3 installs a triggered ability ("Whenever you attack, target
//!   attacking creature gets +1/+1 for each other attacking creature and
//!   gains double strike"). The attacker count is now expressible
//!   (`ObjectFilter::attacking_only()` + script::count_matching, minus the
//!   target by id), but the trigger shape is not: there is no mechanism for
//!   a level-gated triggered ability installed by an activated ability, and
//!   "Whenever you attack" must fire once per combat (CreatureAttacks +
//!   EachTime fires per attacker). The Level 3 trigger remains GAP'd.
//!   // GAP: Level 3 trigger — level-gated triggered-ability installation +
//!   once-per-combat "whenever you attack" condition not available.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Paladin Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::ENCHANTMENT.into(),
        subtypes,
        supertypes: SupertypeSet::default(),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::Level,
                count: 1,
            })
            // Level 2: {2}{W} — Creatures you control get +1/+1
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}: Level 2. Creatures you control get +1/+1.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").unwrap(),
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
            // Level 3: {4}{W}
            .with_activated_ability(ActivatedAbilityDef {
                text: "{4}{W}: Level 3. Whenever you attack, target attacking creature gets +1/+1 for each other attacking creature and gains double strike.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}{W}").unwrap(),
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
                1,
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
    // GAP: Level 3 triggered ability — the "for each other attacking
    // creature" count is now expressible (attacking_only filter), but
    // installing a level-gated "whenever you attack" (once-per-combat)
    // trigger is not; entire Level 3 triggered ability is deferred.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::Level,
        count: 1,
    }]
}
