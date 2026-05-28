//! Forcemage Advocate — `{1}{G}` 2/1 green Centaur Shaman.
//! "{T}: Return target card from an opponent's graveyard to their hand.
//! Put a +1/+1 counter on target creature."
//!
//! GAP: The ability has two independent targets in one activation (a card
//! in opponent's graveyard AND a creature). The engine's target system
//! supports a list of TargetRequirements, but reading two different targets
//! from ctx.targets requires reading index 0 and index 1 with different
//! TargetChoice shapes. The graveyard target is expressed via
//! TargetFilter::Card; the counter target via TargetFilter::Creature.
//! Note: "return from opponent's graveyard to their hand" is modeled with
//! ReturnFromGraveyardToHand — fidelity gap is the "controller pays" check
//! (engine will return to any player's hand, not specifically the opponent's).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Forcemage Advocate");
    let centaur = reg.interner_mut().intern("Centaur");
    let shaman = reg.interner_mut().intern("Shaman");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(centaur);
    subtypes.0.insert(shaman);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}: Return target card from an opponent's graveyard to their hand. Put a +1/+1 counter on target creature.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Card {
                            zone: Zone::Graveyard(0),
                            filter: ObjectFilter::new().controlled_by(ControllerConstraint::Opponent),
                        },
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Creature,
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: advocate_effect,
            }),
    )
}

fn advocate_effect(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(first_target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(graveyard_id) = first_target else {
        return Vec::new();
    };
    let graveyard_id = *graveyard_id;

    let second_target = ctx.targets.targets.get(1);
    let Some(second_target) = second_target else {
        return Vec::new();
    };
    let TargetChoice::Object(creature_id) = second_target else {
        return Vec::new();
    };
    let creature_id = *creature_id;

    vec![
        Effect::ReturnFromGraveyardToHand { target: graveyard_id },
        Effect::AddCounters {
            target: creature_id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}
