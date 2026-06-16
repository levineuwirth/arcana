//! Mu Yanling — `{4}{U}{U}` Legendary Planeswalker — Yanling,
//! starting loyalty 5.
//!
//! +2: Target creature can't be blocked this turn.
//! −3: Draw two cards.
//! −10: Tap all creatures your opponents control. You take an extra turn
//!   after this one.
//!
//! GAP: −10 "tap all creatures your opponents control" — no untargeted
//!   "tap all matching permanents" Effect in the demonstrated surface. The
//!   "take an extra turn" half (Effect::ExtraTurn) IS expressible and is
//!   emitted; the mass-tap half is GAP'd.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mu Yanling");
    let sub = reg.interner_mut().intern("Yanling");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(sub);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            // +2: Target creature can't be blocked this turn.
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Target creature can't be blocked this turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_unblockable,
            })
            // −3: Draw two cards.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Draw two cards.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_draw,
            })
            // −10: Tap all opp creatures (GAP) + take an extra turn.
            .with_activated_ability(ActivatedAbilityDef {
                text: "-10: Tap all creatures your opponents control. You take an extra turn after this one.".into(),
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
                effect: minus_ten_extra_turn,
            }),
    )
}

fn plus_two_unblockable(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::CantBeBlocked {
        target: *id,
        duration: Duration::EndOfTurn,
    }]
}

fn minus_three_draw(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 2 }]
}

fn minus_ten_extra_turn(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "tap all creatures your opponents control" — no mass-tap Effect.
    // The extra-turn half is expressible and emitted.
    vec![Effect::ExtraTurn { player: ctx.controller }]
}
