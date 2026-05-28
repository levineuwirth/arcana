//! Bloodshot Cyclops — `{5}{R}` 4/4 Cyclops Giant.
//! `{T}, Sacrifice a creature: This creature deals damage equal to the sacrificed creature's
//! power to any target.`
//! GAP: ActivationCost has no "sacrifice any creature" cost (only sacrifice self);
//! also the damage amount = sacrificed creature's power is not accessible after sacrifice.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bloodshot Cyclops");
    let cyclops = reg.interner_mut().intern("Cyclops");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cyclops);
    subtypes.0.insert(giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{T}, Sacrifice a creature: This creature deals damage equal to the sacrificed creature's power to any target.".into(),
                cost: ActivationCost {
                    tap: true,
                    // GAP: no "sacrifice any creature" cost field; only sacrifice self
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::any_target()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: deal_damage_by_power,
            }),
    )
}

fn deal_damage_by_power(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: ActivationCost has no "sacrifice any creature" field.
    // GAP: The damage amount = sacrificed creature's power cannot be computed post-sacrifice.
    Vec::new()
}
