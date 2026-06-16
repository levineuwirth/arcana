//! Ugin, the Spirit Dragon — `{8}` Legendary Planeswalker — Ugin, starting
//! loyalty 7. Colorless.
//!
//! +2: Ugin deals 3 damage to any target.
//! −X: Exile each permanent with mana value X or less that's one or more
//!     colors.
//! −10: You gain 7 life, draw seven cards, then put up to seven permanent
//!      cards from your hand onto the battlefield.
//!
//! GAP: −X is a dynamic-X loyalty cost ("−X") — not expressible; omitted.
//! GAP: −10's rider "put up to seven permanent cards from your hand onto the
//!   battlefield" is a variable up-to-seven hand-to-battlefield choice not in
//!   the demonstrated surface; the gain-7-life and draw-seven halves ARE
//!   expressed, the put-onto-battlefield half is dropped (noted here).

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectOrPlayer, TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ugin, the Spirit Dragon");
    let ugin = reg.interner_mut().intern("Ugin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ugin);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{8}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(7),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Ugin deals 3 damage to any target.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_damage,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: You gain 7 life, draw seven cards, then put up to \
                       seven permanent cards from your hand onto the \
                       battlefield."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 10)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_ten,
            }),
    )
    // GAP: -X "Exile each permanent with mana value X or less that's one or
    // more colors" omitted — dynamic-X loyalty cost is not expressible.
}

fn plus_two_damage(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let dt = match target {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(o) => match o {
            ObjectOrPlayer::Object(id) => DamageTarget::Object(*id),
            ObjectOrPlayer::Player(p) => DamageTarget::Player(*p),
        },
    };
    vec![Effect::DealDamage {
        source: ctx.source,
        target: dt,
        amount: 3,
    }]
}

fn minus_ten(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "put up to seven permanent cards from your hand onto the
    // battlefield" is a variable hand-to-battlefield choice not expressible;
    // gain-7-life and draw-seven halves are expressed.
    vec![
        Effect::GainLife {
            player: ctx.controller,
            amount: 7,
        },
        Effect::DrawCards {
            player: ctx.controller,
            count: 7,
        },
    ]
}
