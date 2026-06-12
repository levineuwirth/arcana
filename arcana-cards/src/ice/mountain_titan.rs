//! Mountain Titan — `{2}{B}{R}` 2/2 Giant.
//! `{1}{R}{R}: Until end of turn, whenever you cast a black spell, put a +1/+1 counter on
//! this creature.`

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, ObjectFilter};
use arcana_core::triggers::{PendingTrigger, TriggerCondition};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::{Effect, FloatingUntil};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mountain Titan");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{R}{R}: Until end of turn, whenever you cast a black spell, put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: temporary_trigger,
            }),
    )
}

fn temporary_trigger(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "Until end of turn, whenever you cast a black spell, put a +1/+1
    // counter on this creature."
    vec![Effect::ScheduleFloatingTrigger {
        source: ctx.source,
        controller: ctx.controller,
        condition: TriggerCondition::SpellCast {
            filter: Some(ObjectFilter::new().with_colors(ColorSet::black())),
            caster: ControllerConstraint::You,
        },
        effect: counter_on_black_cast,
        until: FloatingUntil::EndOfTurn,
    }]
}

fn counter_on_black_cast(
    _state: &GameState,
    pt: &PendingTrigger,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: pt.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
