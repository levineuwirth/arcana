//! Vraska, Scheming Gorgon — `{4}{B}{B}` legendary planeswalker, starting loyalty 5.
//!
//! +2: Creatures you control get +1/+0 until end of turn.
//! −3: Destroy target creature.
//! −10: Until end of turn, creatures you control gain deathtouch and a
//!      "lose the game on damage" rider (mass keyword + bespoke trigger GAP).
//!
//! Scope: +2 (controller anthem +1/+0 EOT) and −3 (destroy) are fully
//! expressed. The −10 mass deathtouch + lose-the-game granted ability is
//! GAP'd — no demonstrated mass-keyword-grant + bespoke-trigger Effect.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vraska, Scheming Gorgon");
    let vraska = reg.interner_mut().intern("Vraska");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vraska);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Creatures you control get +1/+0 until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_anthem,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Destroy target creature.".into(),
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
                effect: minus_three_destroy,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−10: Until end of turn, creatures you control gain \
                       deathtouch and \"Whenever this creature deals damage to \
                       an opponent, that player loses the game.\"".into(),
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
                effect: minus_ten_ultimate,
            }),
    )
}

/// `+2: Creatures you control get +1/+0 until end of turn.`
fn plus_two_anthem(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Anthem {
        controller: ctx.controller,
        power: 1,
        toughness: 0,
        duration: Duration::EndOfTurn,
    }]
}

/// `−3: Destroy target creature.`
fn minus_three_destroy(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::DestroyPermanent { target: *id }]
}

/// `−10` — mass deathtouch + lose-game rider.
fn minus_ten_ultimate(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: mass keyword grant + bespoke "loses the game on damage" granted
    // triggered ability has no demonstrated combined Effect.
    Vec::new()
}
