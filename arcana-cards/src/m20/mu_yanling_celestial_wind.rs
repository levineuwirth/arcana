//! Mu Yanling, Celestial Wind — `{4}{U}{U}` Legendary Planeswalker — Yanling,
//! starting loyalty 5. Mono-blue.
//!
//! Oracle:
//! +1: Until your next turn, up to one target creature gets -5/-0.
//! −3: Return up to two target creatures to their owners' hands.
//! −7: Creatures you control with flying get +5/+5 until end of turn.
//!
//! # Scope
//! * +1 — modeled: up to one target creature gets -5/-0 until your next turn.
//! * −3 — modeled: return up to two target creatures to hand.
//! * −7 — modeled: each flying creature you control gets +5/+5 until end of
//!   turn (resolved over the current board).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::effects::KeywordAbility;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mu Yanling, Celestial Wind");
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

    let up_to_one_creature = TargetRequirement {
        filter: TargetFilter::Creature,
        count: TargetCount::UpTo(1),
        controller: None,
    };
    let up_to_two_creatures = TargetRequirement {
        filter: TargetFilter::Creature,
        count: TargetCount::UpTo(2),
        controller: None,
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Until your next turn, up to one target creature gets \
                       -5/-0.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![up_to_one_creature],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one_shrink,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−3: Return up to two target creatures to their owners' \
                       hands.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![up_to_two_creatures],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_bounce,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "−7: Creatures you control with flying get +5/+5 until \
                       end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 7)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_seven_pump,
            }),
    )
}

/// `+1:` up to one target creature gets -5/-0 until your next turn.
fn plus_one_shrink(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::Pump {
        target: *id,
        power: -5,
        toughness: 0,
        duration: Duration::UntilYourNextTurn(ctx.controller),
        keywords: vec![],
    }]
}

/// `−3:` return up to two target creatures to hand.
fn minus_three_bounce(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    ctx.targets
        .targets
        .iter()
        .filter_map(|t| match t {
            TargetChoice::Object(id) => Some(Effect::ReturnToHand { target: *id }),
            _ => None,
        })
        .collect()
}

/// `−7:` your flying creatures get +5/+5 until end of turn.
fn minus_seven_pump(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let filter = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You)
        .with_keyword(KeywordAbility::Flying);
    script::ids_matching(state, &filter, ctx.controller)
        .into_iter()
        .map(|id| Effect::Pump {
            target: id,
            power: 5,
            toughness: 5,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        })
        .collect()
}
