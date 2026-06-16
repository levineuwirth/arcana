//! Vivien of the Arkbow — `{4}{G}{G}` Legendary Planeswalker — Vivien.
//! Starting loyalty 5 (oracle).
//!
//! +2: Put two +1/+1 counters on up to one target creature.
//! −3: Target creature you control deals damage equal to its power to target
//!     creature you don't control.
//! −9: Creatures you control get +4/+4 and gain trample until end of turn.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::events::DamageTarget;
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
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount,
    TargetFilter, TargetRequirement,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vivien of the Arkbow");
    let vivien = reg.interner_mut().intern("Vivien");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vivien);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(5),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+2: Put two +1/+1 counters on up to one target \
                       creature.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_two_counters,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Target creature you control deals damage equal to \
                       its power to target creature you don't control.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 3)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature()
                                .controlled_by(ControllerConstraint::Opponent),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three_bite,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-9: Creatures you control get +4/+4 and gain trample \
                       until end of turn.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 9)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_nine_anthem,
            }),
    )
}

fn plus_two_counters(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(id)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    vec![Effect::AddCounters {
        target: *id,
        kind: CounterKind::PlusOnePlusOne,
        count: 2,
    }]
}

fn minus_three_bite(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut targets = ctx.targets.targets.iter();
    let Some(TargetChoice::Object(source_creature)) = targets.next() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(victim)) = targets.next() else {
        return Vec::new();
    };
    let power = script::power_of(state, *source_creature).max(0) as u32;
    vec![Effect::DealDamage {
        source: *source_creature,
        target: DamageTarget::Object(*victim),
        amount: power,
    }]
}

fn minus_nine_anthem(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let your_creatures = ObjectFilter::creature()
        .controlled_by(ControllerConstraint::You);
    let ids = script::ids_matching(state, &your_creatures, ctx.controller);
    let mut effects = vec![Effect::Anthem {
        controller: ctx.controller,
        power: 4,
        toughness: 4,
        duration: Duration::EndOfTurn,
    }];
    for id in ids {
        effects.push(Effect::GrantKeyword {
            target: id,
            keyword: KeywordAbility::Trample,
            duration: Duration::EndOfTurn,
        });
    }
    effects
}
