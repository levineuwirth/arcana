//! Nicol Bolas, the Deceiver — `{5}{U}{B}{R}` Legendary Planeswalker — Bolas,
//! starting loyalty 7.
//!
//! +3: Each opponent loses 3 life unless that player sacrifices a nonland
//!     permanent of their choice or discards a card.
//! −3: Destroy target creature. Draw a card.
//! −11: Nicol Bolas deals 7 damage to each opponent. You draw seven cards.
//!
//! GAP: +3 "loses 3 life unless that player sacrifices a nonland permanent or
//!   discards a card" — this is a COMPOUND sac-OR-discard payment (two
//!   alternative payment modes). OptionalPaymentKind is a single cost, so the
//!   "sacrifice OR discard" choice is not expressible faithfully; wiring only
//!   one mode would silently drop the other. The +3 ability shell is declared
//!   with the correct cost and returns Vec::new().

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Nicol Bolas, the Deceiver");
    let bolas = reg.interner_mut().intern("Bolas");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(bolas);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{U}{B}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(7),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+3: Each opponent loses 3 life unless that player \
                       sacrifices a nonland permanent of their choice or \
                       discards a card."
                    .into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_three_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Destroy target creature. Draw a card.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-11: Nicol Bolas deals 7 damage to each opponent. You \
                       draw seven cards."
                    .into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 11)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_eleven,
            }),
    )
}

fn plus_three_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "loses 3 life unless sacrifices a nonland permanent OR discards a
    // card" — a compound sac-OR-discard payment (two alternative modes).
    // OptionalPaymentKind is a single cost, so this isn't expressible.
    Vec::new()
}

fn minus_three(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![
        Effect::DestroyPermanent { target: *id },
        Effect::DrawCards {
            player: ctx.controller,
            count: 1,
        },
    ]
}

fn minus_eleven(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects: Vec<Effect> = script::opponents(state, ctx.controller)
        .into_iter()
        .map(|opp| Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(opp),
            amount: 7,
        })
        .collect();
    effects.push(Effect::DrawCards {
        player: ctx.controller,
        count: 7,
    });
    effects
}
