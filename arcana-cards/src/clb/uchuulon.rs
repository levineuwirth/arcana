//! Uchuulon — `{3}{B}` */4 Crab Ooze Horror.
//! "Uchuulon's power is equal to the number of Crabs, Oozes, and/or Horrors you
//! control."
//! "Horrific Symbiosis — At the beginning of your end step, exile up to one
//! target creature card from an opponent's graveyard. If you do, create a token
//! that's a copy of this creature."
//!
//! The power is `*` (PtValue::Star): an ASYMMETRIC subtype CDA — power = the
//! number of Crabs, Oozes, and/or Horrors you control, toughness fixed 4 —
//! wired at Layer 7a via `self_pt_from_match_asym` over a multi-subtype filter
//! (the Crab/Ooze/Horror symbols are resolved by name in the install fn, which
//! has the interner). The end-step ability is implemented: exile up to one
//! target creature card from an opponent's graveyard, then create a token copy
//! of this creature.

use arcana_core::effects::Effect;
use arcana_core::layers::{ContinuousEffect, Duration};
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
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
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
            })
            .with_triggered_ability(TriggeredAbilityDef {
                id: 2,
                trigger_condition: TriggerCondition::SelfEntersBattlefield,
                intervening_if: None,
                effect: install_cda,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            }),
    )
}

/// Power = Crabs/Oozes/Horrors you control; toughness fixed 4.
fn install_cda(_s: &GameState, trig: &PendingTrigger, reg: &CardRegistry) -> Vec<Effect> {
    let syms: Vec<u32> = ["Crab", "Ooze", "Horror"]
        .iter()
        .filter_map(|s| reg.interner().lookup(s))
        .collect();
    let filter = ObjectFilter::creature()
        .with_subtypes_any(syms)
        .controlled_by(ControllerConstraint::You);
    vec![Effect::InstallContinuousEffect {
        effect: ContinuousEffect::self_pt_from_match_asym(
            trig.source,
            filter,
            /*count_is_power=*/ true,
            /*other_fixed=*/ 4,
            Duration::WhileSourceOnBattlefield,
        ),
    }]
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
