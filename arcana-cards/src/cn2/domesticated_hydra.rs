//! Domesticated Hydra — `{2}{G}{G}` 3/3 Creature — Hydra. Green.
//! "{X}{G}{G}{G}: Monstrosity X." — approximated as putting X +1/+1 counters
//! on this creature (the "becomes monstrous" flag is unmodeled).
//! "As long as this creature is monstrous, it has trample." — a monstrous-flag
//! gated continuous ability, not expressible (GAP).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Domesticated Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        // GAP: Monstrosity is not a usable KeywordAbility variant.
        keywords: vec![],
        ..Default::default()
    };

    // GAP: static — "as long as this creature is monstrous, it has trample"
    // is a monstrous-flag-gated continuous ability, not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{X}{G}{G}{G}: Monstrosity X.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{X}{G}{G}{G}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: monstrosity_x,
        }),
    )
}

fn monstrosity_x(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "becomes monstrous" flag is unmodeled; only the X +1/+1
    // counters part of Monstrosity X is expressed.
    let n = ctx.x_value.unwrap_or(0);
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: n,
    }]
}
