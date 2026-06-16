//! Vivien, Arkbow Ranger — `{1}{G}{G}{G}` Legendary Planeswalker — Vivien,
//! starting loyalty 4.
//!
//! +1: Distribute two +1/+1 counters among up to two target creatures. They
//!     gain trample until end of turn. (Modeled as one +1/+1 counter on each
//!     chosen target plus trample; the free distribution choice — e.g. both
//!     onto one creature — is approximated as the even 1+1 split.)
//! −3: Target creature you control deals damage equal to its power to target
//!     creature or planeswalker.
//! −5: You may reveal a creature card you own from outside the game and put it
//!     into your hand. GAP: "outside the game" (sideboard/wish) has no zone.

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
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vivien, Arkbow Ranger");
    let vivien = reg.interner_mut().intern("Vivien");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vivien);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(4),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Distribute two +1/+1 counters among up to two target \
                       creatures. They gain trample until end of turn.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Creature,
                    count: TargetCount::UpTo(2),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: plus_one,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-3: Target creature you control deals damage equal to its \
                       power to target creature or planeswalker.".into(),
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
                        controller: Some(ControllerConstraint::You),
                    },
                    TargetRequirement::target_creature(),
                ],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_three,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-5: You may reveal a creature card you own from outside \
                       the game and put it into your hand.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 5)),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: minus_five,
            }),
    )
}

fn plus_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // One +1/+1 counter on each chosen target (even split of the two
    // counters) plus trample until end of turn.
    let mut out = Vec::new();
    for choice in ctx.targets.targets.iter() {
        if let TargetChoice::Object(id) = choice {
            out.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::PlusOnePlusOne,
                count: 1,
            });
            out.push(Effect::GrantKeyword {
                target: *id,
                keyword: KeywordAbility::Trample,
                duration: Duration::EndOfTurn,
            });
        }
    }
    out
}

fn minus_three(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(source_creature)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let Some(target_choice) = ctx.targets.targets.get(1) else {
        return Vec::new();
    };
    let dt = match target_choice {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        _ => return Vec::new(),
    };
    let amount = script::power_of(state, *source_creature).max(0) as u32;
    vec![Effect::DealDamage {
        source: *source_creature,
        target: dt,
        amount,
    }]
}

fn minus_five(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "from outside the game" (wish/sideboard) has no modeled zone.
    Vec::new()
}
