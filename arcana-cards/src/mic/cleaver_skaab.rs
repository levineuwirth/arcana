//! Cleaver Skaab — `{3}{U}` 2/4 blue Zombie Horror. "{3}, {T}, Sacrifice another
//! Zombie: Create two tokens that are copies of the sacrificed creature."
//! GAP: "Sacrifice another Zombie" is not in ActivationCost. CopyPermanent targets
//! a battlefield creature, but the sacrificed creature is already gone.

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
    let name = reg.interner_mut().intern("Cleaver Skaab");
    let zombie = reg.interner_mut().intern("Zombie");
    let horror = reg.interner_mut().intern("Horror");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(horror);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}, {T}, Sacrifice another Zombie: Create two tokens that are copies of the sacrificed creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}").unwrap(),
                    tap: true,
                    // GAP: "Sacrifice another Zombie (not self)" not in ActivationCost
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: copy_zombie_tokens,
            }),
    )
}

fn copy_zombie_tokens(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Creating tokens that are copies of the sacrificed creature (which is
    // now in graveyard) is not expressible in the Effect catalog.
    Vec::new()
}
