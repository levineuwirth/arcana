//! Combine Guildmage — `{G}{U}` 2/2 Merfolk Wizard.
//! "{1}{G}, {T}: This turn, each creature you control enters with an
//!  additional +1/+1 counter on it."
//! "{1}{U}, {T}: Move a +1/+1 counter from target creature you control
//!  onto another target creature you control."
//!
//! The first activation installs a turn-long replacement effect on
//! entering creatures, which has no demonstrated primitive — GAP'd.
//! The second moves a counter, expressed as RemoveCounters from the
//! first target + AddCounters to the second, wrapped in a Sequence.

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

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Combine Guildmage");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{G}, {T}: This turn, each creature you control enters \
                       with an additional +1/+1 counter on it."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{G}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: extra_counter_replacement,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}, {T}: Move a +1/+1 counter from target creature you \
                       control onto another target creature you control."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement {
                        filter: TargetFilter::Permanent(
                            ObjectFilter::creature().controlled_by(ControllerConstraint::You),
                        ),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                ],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: move_counter,
            }),
    )
}

fn extra_counter_replacement(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "this turn, each creature you control enters with an additional
    // +1/+1 counter" — a turn-scoped enters-with replacement effect; no
    // demonstrated primitive installs this.
    Vec::new()
}

fn move_counter(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(from)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let Some(TargetChoice::Object(to)) = ctx.targets.targets.get(1) else {
        return Vec::new();
    };
    vec![Effect::Sequence(vec![
        Effect::RemoveCounters {
            target: *from,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::AddCounters {
            target: *to,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ])]
}
