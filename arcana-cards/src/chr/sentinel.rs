//! Sentinel — `{4}` 1/1 colorless Artifact Creature — Shapeshifter.
//! "{0}: Change this creature's base toughness to 1 plus the power of target
//! creature blocking or blocked by this creature. (This effect lasts
//! indefinitely.)"
//!
//! GAP: no "set toughness equal to 1 + power of another blocking/blocked
//! creature" Effect variant (requires dynamic computation and no EndOfTurn
//! duration for the indefinite form).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sentinel");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{0}: Change this creature's base toughness to 1 plus the power of target creature blocking or blocked by this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{0}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: change_toughness_to_blocker_power,
            }),
    )
}

fn change_toughness_to_blocker_power(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: no "set toughness = 1 + target's power indefinitely" Effect variant.
    Vec::new()
}
