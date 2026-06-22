//! Gristleback — `{2}{G}` 2/2 Boar Beast with Bloodthirst 1.
//! "Bloodthirst 1 (If an opponent was dealt damage this turn, this creature
//! enters with a +1/+1 counter on it.)"
//! "Sacrifice this creature: You gain life equal to this creature's power."
//!
//! Bloodthirst is a fully-implemented parametrized keyword. The sacrifice
//! ability gains life equal to the creature's power, read at resolution.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gristleback");
    let boar = reg.interner_mut().intern("Boar");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);
    subtypes.0.insert(beast);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Bloodthirst(1)],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "Sacrifice this creature: You gain life equal to this creature's power.".into(),
            cost: ActivationCost {
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: sacrifice_gain_life,
        }),
    )
}

fn sacrifice_gain_life(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let amount = script::power_of(state, ctx.source).max(0) as u32;
    vec![Effect::GainLife { player: ctx.controller, amount }]
}
