//! Gideon Jura — `{3}{W}{W}` Legendary Planeswalker — Gideon, starting loyalty 6.
//!
//! +2: During target opponent's next turn, creatures that player controls attack
//!   Gideon Jura if able. GAP: a delayed "must attack this planeswalker" combat-
//!   requirement over an opponent's next turn is not expressible; shell declared
//!   at the correct +2 cost.
//! −2: Destroy target tapped creature. Modeled with `Effect::DestroyPermanent`
//!   over a tapped-creature target filter.
//! 0: Until end of turn, Gideon Jura becomes a 6/6 Human Soldier creature that's
//!   still a planeswalker. Prevent all damage that would be dealt to him this
//!   turn. The damage prevention on Gideon is modeled (`Effect::PreventDamage`,
//!   EndOfTurn); GAP: the "becomes a 6/6 Human Soldier creature" self-animation
//!   (type-add + base-P/T while staying a planeswalker) is not expressible.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::replacement::ReplacementDuration;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gideon Jura");
    let gideon = reg.interner_mut().intern("Gideon");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gideon);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: During target opponent's next turn, creatures that \
                       player controls attack Gideon Jura if able.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_player()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_gap,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-2: Destroy target tapped creature.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
                        tapped: Some(true),
                        ..Default::default()
                    }),
                    count: TargetCount::Exactly(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_two_destroy_tapped,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "0: Until end of turn, Gideon Jura becomes a 6/6 Human \
                       Soldier creature that's still a planeswalker. Prevent all \
                       damage that would be dealt to him this turn.".into(),
                cost: ActivationCost::default(),
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: zero_animate_prevent,
            }),
    )
}

fn plus_two_gap(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: delayed "creatures the opponent controls attack Gideon if able" combat
    //      requirement over the opponent's next turn is not expressible.
    Vec::new()
}

fn minus_two_destroy_tapped(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else { return Vec::new(); };
    vec![Effect::DestroyPermanent { target: *id }]
}

fn zero_animate_prevent(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "becomes a 6/6 Human Soldier creature that's still a planeswalker"
    //      self-animation not expressible. The damage prevention IS modeled.
    vec![Effect::PreventDamage {
        target: DamageTarget::Object(ctx.source),
        amount: None,
        duration: ReplacementDuration::EndOfTurn,
    }]
}
