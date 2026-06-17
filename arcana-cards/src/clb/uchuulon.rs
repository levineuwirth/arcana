//! Uchuulon — `{3}{B}` */4 Crab Ooze Horror.
//! "Uchuulon's power is equal to the number of Crabs, Oozes, and/or Horrors you
//! control."
//! "Horrific Symbiosis — At the beginning of your end step, exile up to one
//! target creature card from an opponent's graveyard. If you do, create a token
//! that's a copy of this creature."
//!
//! The power is `*` (PtValue::Star). GAP: the characteristic-defining ability
//! that sets the star value to the count of Crabs/Oozes/Horrors you control is
//! not expressible (no documented CDA wiring) — base Star is emitted.
//! The end-step ability is implemented: exile up to one target creature card
//! from an opponent's graveyard, then create a token copy of this creature.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::targets::{
    ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter,
    TargetRequirement,
};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::turn::Step;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Uchuulon");
    let crab = reg.interner_mut().intern("Crab");
    let ooze = reg.interner_mut().intern("Ooze");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(crab);
    subtypes.0.insert(ooze);
    subtypes.0.insert(horror);

    // GAP: CDA setting power = number of Crabs/Oozes/Horrors you control — base Star emitted.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_triggered_ability(TriggeredAbilityDef {
            id: 1,
            trigger_condition: TriggerCondition::StepBegins {
                step: Step::End,
                whose: ControllerConstraint::You,
            },
            intervening_if: None,
            effect: exile_gy_creature_and_copy_self,
            trigger_zones: vec![Zone::Battlefield],
            frequency: TriggerFrequency::EachTime,
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Card {
                    zone: Zone::Graveyard(0),
                    filter: ObjectFilter::creature()
                        .controlled_by(ControllerConstraint::Opponent),
                },
                count: TargetCount::UpTo(1),
                controller: None,
            }],
        }),
    )
}

fn exile_gy_creature_and_copy_self(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = trig.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![
        Effect::ExileFromGraveyard { target: *id },
        Effect::CopyPermanent { target: trig.source },
    ]
}
