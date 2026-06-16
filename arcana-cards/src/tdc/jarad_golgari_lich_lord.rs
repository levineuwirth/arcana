//! Jarad, Golgari Lich Lord — `{B}{B}{G}{G}` 2/2 Legendary Zombie Elf.
//! "Jarad gets +1/+1 for each creature card in your graveyard.
//!  {1}{B}{G}, Sacrifice another creature: Each opponent loses life
//!  equal to the sacrificed creature's power.
//!  Sacrifice a Swamp and a Forest: Return this card from your
//!  graveyard to your hand."
//!
//! GAP (static): "Jarad gets +1/+1 for each creature card in your
//! graveyard" is a self-buffing characteristic-defining static — not a
//! triggered or activated ability and not expressible with the demonstrated
//! effect surface.
//! GAP (third ability): "Sacrifice a Swamp and a Forest" requires paying
//! TWO distinct typed sacrifices as a cost; ActivationCost.sacrifice_other
//! carries a single filter only, so the two-permanent cost is inexpressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{
    CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Jarad, Golgari Lich Lord");
    let zombie = reg.interner_mut().intern("Zombie");
    let elf = reg.interner_mut().intern("Elf");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(elf);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{G}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{B}{G}, Sacrifice another creature: Each opponent loses life equal to the sacrificed creature's power.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{B}{G}").expect("valid cost"),
                sacrifice_other: Some(ObjectFilter {
                    types: Some(TypeLine::CREATURE.into()),
                    ..ObjectFilter::default()
                }),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: each_opponent_loses_life,
        }),
    )
}

fn each_opponent_loses_life(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: amount = power of the creature sacrificed as a cost. No documented
    // accessor exposes the cost-sacrificed permanent's power to the resolver,
    // so the dynamic life loss can't be computed.
    Vec::new()
}
