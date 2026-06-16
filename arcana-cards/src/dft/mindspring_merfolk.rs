//! Mindspring Merfolk — `{U}` 1/1 blue Merfolk Wizard.
//! "Exhaust — {X}{U}{U}, {T}: Draw X cards. Put a +1/+1 counter on each Merfolk
//! creature you control. (Activate each exhaust ability only once.)"
//! GAP: Exhaust (once-only activation marker) not modeled in ActivatedAbilityDef;
//! emitting as a normal activated ability. X-cost not supported via ManaCost::parse;
//! using {U}{U} as placeholder.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ControllerConstraint;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mindspring Merfolk");
    let merfolk = reg.interner_mut().intern("Merfolk");
    let wizard = reg.interner_mut().intern("Wizard");
    let _merfolk_st = reg.interner_mut().intern("Merfolk");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(merfolk);
    subtypes.0.insert(wizard);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Exhaust — {X}{U}{U}, {T}: Draw X cards. Put a +1/+1 counter on each Merfolk creature you control.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{U}{U}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: exhaust_draw_and_counters,
            }),
    )
}

fn exhaust_draw_and_counters(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let x = ctx.x_value.unwrap_or(0);
    let merfolk_filter = script::subtype_filter(reg, "Merfolk")
        .controlled_by(ControllerConstraint::You);
    let merfolk_ids = script::ids_matching(state, &merfolk_filter, ctx.controller);
    let mut effects = vec![Effect::DrawCards { player: ctx.controller, count: x }];
    for id in merfolk_ids {
        effects.push(Effect::AddCounters {
            target: id,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        });
    }
    effects
}
