//! Sorin Markov — `{3}{B}{B}{B}` legendary planeswalker, starting loyalty 4.
//! Subtype Sorin.
//!
//! # Loyalty abilities
//!
//! * `+2`: Sorin Markov deals 2 damage to any target and you gain 2
//!   life. (DealDamage to an any-target choice + GainLife.)
//! * `−3`: Target opponent's life total becomes 10. (SetLifeTotal.)
//! * `−7`: You control target player during that player's next turn.
//!   GAP — "control another player's turn" has no demonstrated Effect
//!   surface; the ability shell is declared with the correct cost.
//!
//! # Rules references
//!
//! * CR 113.3c — enters with loyalty counters equal to printed loyalty.
//! * CR 606 / 606.3 — loyalty abilities (sorcery speed, controller-only,
//!   one per turn per planeswalker).
//! * CR 704.5i — 0-loyalty state-based sacrifice.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectOrPlayer, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sorin Markov");
    let sorin = reg.interner_mut().intern("Sorin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sorin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Sorin Markov deals 2 damage to any target and \
                       you gain 2 life.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Target opponent's life total becomes 10.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Player,
                    count: TargetCount::Exactly(1),
                    controller: Some(ControllerConstraint::Opponent),
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: You control target player during that player's \
                       next turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: arcana_core::registry::ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven,
            }),
    )
}

/// `+2: Sorin Markov deals 2 damage to any target and you gain 2 life.`
fn plus_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
            DamageTarget::Object(*id)
        }
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
            DamageTarget::Player(*p)
        }
        _ => return Vec::new(),
    };
    vec![
        Effect::DealDamage {
            source: ctx.source,
            target: dt,
            amount: 2,
        },
        Effect::GainLife {
            player: ctx.controller,
            amount: 2,
        },
    ]
}

/// `−3: Target opponent's life total becomes 10.`
fn minus_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::SetLifeTotal {
        player: *p,
        amount: 10,
    }]
}

/// `−7: You control target player during that player's next turn.`
fn minus_seven(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you control target player during their next turn" has no
    // demonstrated Effect variant; ability shell declared at correct cost.
    Vec::new()
}
