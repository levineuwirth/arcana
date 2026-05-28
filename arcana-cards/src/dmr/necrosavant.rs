//! Necrosavant — `{3}{B}{B}{B}` 5/5 Zombie Giant.
//! `{3}{B}{B}, Sacrifice a creature: Return this card from your graveyard to the battlefield.
//! Activate only during your upkeep.`
//! GAP: "Activate only during your upkeep" — timing restriction not expressible.
//! GAP: Activating from graveyard requires ActivationZone::Graveyard (not in catalog;
//! using Battlefield as fallback).
//! Effect: "return this card from your graveyard to the battlefield" — no self-reanimate
//! Effect variant (ReturnFromGraveyardToBattlefield needs a target id, not self).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::effects::Effect;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Necrosavant");
    let zombie = reg.interner_mut().intern("Zombie");
    let giant = reg.interner_mut().intern("Giant");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(giant);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(5)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{B}{B}, Sacrifice a creature: Return this card from your graveyard to the battlefield. Activate only during your upkeep.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{B}{B}").unwrap(),
                    sacrifice: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: self_reanimate,
            }),
    )
}

fn self_reanimate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return this card from your graveyard to the battlefield" — no self-reanimate
    // Effect; ReturnFromGraveyardToBattlefield needs a target id, not self-reference.
    // GAP: ActivationZone::Graveyard not in catalog; ability fires from battlefield.
    Vec::new()
}
