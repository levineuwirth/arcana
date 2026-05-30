//! Barbarian Class — `{R}` Enchantment — Class.
//!
//! Level 1 (base): If you would roll one or more dice, instead roll that many
//! dice plus one and ignore the lowest roll.
//! GAP: per-level static (dice replacement) is a continuous-effect engine gap —
//!   replacement effects for dice rolls are not modeled.
//!
//! {1}{R}: Level 2. Whenever you roll one or more dice, target creature you
//! control gets +2/+0 and gains menace until end of turn.
//! GAP: "whenever you roll dice" trigger condition not modeled (no DiceRolled
//!   GameEvent in the engine).
//!
//! {2}{R}: Level 3. Creatures you control have haste.
//!   Modeled via keyword anthem installed on level-up.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::{ContinuousEffect, Duration};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Barbarian Class");
    let class_sub = reg.interner_mut().intern("Class");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(class_sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
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
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}: Level 2.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
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
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{R}: Level 3.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").expect("valid cost"),
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
    // GAP: level-2 "whenever you roll dice" triggered ability not modeled
    // (no DiceRolled event in the engine).
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
                KeywordAbility::Haste,
                Duration::WhileSourceOnBattlefield,
            ),
        },
    ]
}
