//! Coffin Puppets — `{3}{B}{B}` 3/3 Creature — Zombie.
//! `Sacrifice two lands: Return this card from your graveyard to the battlefield. Activate
//!  only during your upkeep and only if you control a Swamp.`
//! "Sacrifice two lands" cost modeled via `sacrifice_other` (land filter) +
//! `sacrifice_other_count: 2`.
//! GAP: "only during your upkeep and only if you control a Swamp" timing/
//! condition not enforced.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Coffin Puppets");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice two lands: Return this card from your graveyard to the battlefield. Activate only during your upkeep and only if you control a Swamp.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(arcana_core::targets::ObjectFilter {
                        types: Some(TypeLine::LAND.into()),
                        ..Default::default()
                    }),
                    sacrifice_other_count: 2,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Graveyard,
                is_instant_speed: false,
                face_gate: None,
                effect: return_from_graveyard,
            }),
    )
}

fn return_from_graveyard(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice two lands" cost not enforced; "only if you control a Swamp" not checked
    vec![Effect::ReturnFromGraveyardToBattlefield { target: ctx.source }]
}
