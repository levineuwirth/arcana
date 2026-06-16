//! Fleecemane Lion — `{G}{W}` 3/3 Cat.
//! {3}{G}{W}: Monstrosity 1. (Approximated as putting a +1/+1 counter on
//! this creature; the "becomes monstrous" flag is unmodeled.)
//! As long as this creature is monstrous, it has hexproof and indestructible
//! (GAP — monstrous-gated static not expressible).

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
    let name = reg.interner_mut().intern("Fleecemane Lion");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);

    // "Monstrosity" is not a usable KeywordAbility variant.
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{W}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    // GAP: static — "as long as monstrous, has hexproof and indestructible"
    // is a monstrous-flag-gated continuous ability, not expressible.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}{G}{W}: Monstrosity 1.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}{G}{W}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: monstrosity_one,
        }),
    )
}

fn monstrosity_one(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: the "becomes monstrous" flag is unmodeled; only the +1/+1 counter
    // part of Monstrosity 1 is expressed.
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
