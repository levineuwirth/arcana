//! Syr Ginger, the Meal Ender — `{2}` Legendary 3/1 Artifact Creature —
//! Food Knight.
//! Syr Ginger has trample, hexproof, and haste as long as an opponent
//! controls a planeswalker.
//! Whenever another artifact you control is put into a graveyard from the
//! battlefield, put a +1/+1 counter on Syr Ginger and scry 1.
//! {2}, {T}, Sacrifice Syr Ginger: You gain life equal to its power.
//!
//! Note: Scryfall lists "Scry" as a keyword, but Scry is an effect
//! (`Effect::Scry`), not a `KeywordAbility` variant — the keyword line is
//! empty and the scry is wired into the dies trigger.
//! GAP: the conditional static "has trample, hexproof, and haste as long as
//! an opponent controls a planeswalker" is a condition-gated keyword grant
//! with no demonstrated static primitive — omitted.
//! GAP (partial): the "another" self-exclusion on the artifact-dies trigger
//! filter is not expressible (no self-id predicate); the trigger fires for
//! any artifact you control put into a graveyard from the battlefield.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{
    PendingTrigger, TriggerCondition, TriggerFrequency, TriggeredAbilityDef,
};
use arcana_core::types::{
    CardId, ColorSet, CounterKind, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};
use arcana_core::zones::Zone;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Syr Ginger, the Meal Ender");
    let food = reg.interner_mut().intern("Food");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(food);
    subtypes.0.insert(knight);

    let artifact_dies = ObjectFilter::permanent()
        .with_types(TypeLine::ARTIFACT.into())
        .controlled_by(ControllerConstraint::You);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_triggered_ability(TriggeredAbilityDef {
                id: 1,
                trigger_condition: TriggerCondition::ZoneChange {
                    filter: artifact_dies,
                    from: Some(Zone::Battlefield),
                    to: Zone::Graveyard(0),
                },
                intervening_if: None,
                effect: counter_and_scry,
                trigger_zones: vec![Zone::Battlefield],
                frequency: TriggerFrequency::EachTime,
                target_requirements: Vec::new(),
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}, {T}, Sacrifice Syr Ginger: You gain life equal to \
                       its power."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    tap: true,
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_life_equal_to_power,
            }),
    )
}

fn counter_and_scry(
    _state: &GameState,
    trig: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::AddCounters {
            target: trig.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
        Effect::Scry {
            player: trig.controller,
            count: 1,
        },
    ]
}

fn gain_life_equal_to_power(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let n = script::power_of(state, ctx.source).max(0) as u32;
    vec![Effect::GainLife {
        player: ctx.controller,
        amount: n,
    }]
}
