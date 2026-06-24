//! Ob Nixilis, the Adversary — `{1}{B}{R}` Legendary Planeswalker — Nixilis, starting loyalty 5.
//!
//! Casualty X (Scryfall keyword): GAP — the casualty cast mechanic and the
//!   copy-with-starting-loyalty-X rider are not expressible from the demonstrated
//!   surface (and `keywords: vec![]` for planeswalker class per system prompt).
//! +1: Each opponent loses 2 life unless they discard a card. If you control a
//!   Demon or Devil, you gain 2 life. The per-opponent "lose 2 life unless they
//!   discard a card" is wired as one OptionalPayment per opponent (chooser =
//!   that opponent, cost = Discard(1), else_effect = lose 2 life); the
//!   conditional 2-life gain is modeled.
//! −2: Create a 1/1 red Devil creature token with "When this token dies, it deals
//!   1 damage to any target."
//! −7: Target player draws seven cards and loses 7 life.

use arcana_core::actions::OptionalPaymentKind;
use arcana_core::effects::{Condition, Effect, TokenDefinition};
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetRequirement};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ob Nixilis, the Adversary");
    let nixilis = reg.interner_mut().intern("Nixilis");
    let _demon = reg.interner_mut().intern("Demon");
    let _devil = reg.interner_mut().intern("Devil");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(nixilis);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        keywords: vec![],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Each opponent loses 2 life unless they discard a card. \
                       If you control a Demon or Devil, you gain 2 life.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_drain,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Create a 1/1 red Devil creature token with \"When this \
                       token dies, it deals 1 damage to any target.\"".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_devil,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-7: Target player draws seven cards and loses 7 life.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_draw_lose,
            }),
    )
}

fn plus_one_drain(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = Vec::new();
    // "Each opponent loses 2 life unless they discard a card." One
    // OptionalPayment per opponent: the chooser is that opponent, paying =
    // discard one card (avoiding the penalty), declining = lose 2 life.
    for opp in script::opponents(state, ctx.controller) {
        effects.push(Effect::OptionalPayment {
            chooser: opp,
            cost: OptionalPaymentKind::Discard(1),
            then: Box::new(Effect::Sequence(vec![])),
            else_effect: Some(Box::new(Effect::LoseLife { player: opp, amount: 2 })),
        });
    }
    // If you control a Demon or Devil, you gain 2 life.
    let demon = reg.interner().lookup("Demon");
    let devil = reg.interner().lookup("Devil");
    let mut subtypes_any = Vec::new();
    if let Some(d) = demon { subtypes_any.push(d); }
    if let Some(d) = devil { subtypes_any.push(d); }
    effects.push(Effect::Conditional {
        condition: Condition::ControlPermanentMatching(ObjectFilter {
            subtypes_any: Some(subtypes_any),
            ..Default::default()
        }),
        then: Box::new(Effect::GainLife { player: ctx.controller, amount: 2 }),
        otherwise: None,
    });
    effects
}

fn minus_two_devil(
    _state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let devil = reg.interner().lookup("Devil").expect("Devil interned at register");
    let mut token_subtypes = SubtypeSet::default();
    token_subtypes.0.insert(devil);
    vec![Effect::CreateToken {
        controller: ctx.controller,
        token: TokenDefinition {
            name: devil,
            colors: ColorSet::red(),
            types: TypeLine::CREATURE.into(),
            subtypes: token_subtypes,
            power: Some(PtValue::Fixed(1)),
            toughness: Some(PtValue::Fixed(1)),
            keywords: vec![],
            abilities: vec![TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::SelfDies,
                intervening_if: None,
                effect: devil_dies_damage,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: vec![TargetRequirement::any_target()],
            }],
        },
    }]
}

fn devil_dies_damage(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else { return Vec::new(); };
    let dt = match target {
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        _ => return Vec::new(),
    };
    vec![Effect::DealDamage { source: trig.source, target: dt, amount: 1 }]
}

fn minus_seven_draw_lose(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![
        Effect::DrawCards { player: *p, count: 7 },
        Effect::LoseLife { player: *p, amount: 7 },
    ]
}
