//! Drillworks Mole — `{1}` 1/1 colorless Artifact Creature — Mole.
//! "{2}, {T}: Put a +1/+1 counter on this creature and a +1/+1 counter on
//! up to one target commander creature you control."
//! GAP: "target commander creature" — no Commander supertype filter in
//! ObjectFilter; targeting any creature you control as proxy.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Drillworks Mole");
    let mole = reg.interner_mut().intern("Mole");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(mole);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}: Put a +1/+1 counter on this creature and a +1/+1 counter on up to one target commander creature you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Permanent(
                        ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                    ),
                    count: TargetCount::UpTo(1),
                    controller: None,
                }],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: add_counters,
            }),
    )
}

fn add_counters(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let mut effects = vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }];
    if let Some(target) = ctx.targets.targets.first() {
        if let TargetChoice::Object(id) = target {
            if *id != ctx.source {
                effects.push(Effect::AddCounters {
                    target: *id,
                    kind: CounterKind::PlusOnePlusOne,
                    count: 1,
                });
            }
        }
    }
    effects
}
