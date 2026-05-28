//! Vladimir and Godfrey — `{2}{W}` 3/2 Legendary Zombie Knight.
//! Rejuvenation — `{2}{W}: Return Vladimir and Godfrey from your graveyard to the battlefield tapped.
//! It perpetually gets +1/+1. Activate only if you control a 1/1 creature.`
//! GAP: ActivationZone::Graveyard is not a recognized ActivationZone variant; no way to activate
//! from graveyard. Also "perpetually gets +1/+1" (permanent counter for Arena) has no Effect variant.
//! Also the "activate only if you control a 1/1 creature" condition cannot be gated.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vladimir and Godfrey");
    let zombie = reg.interner_mut().intern("Zombie");
    let knight = reg.interner_mut().intern("Knight");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    subtypes.0.insert(knight);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W}: Return Vladimir and Godfrey from your graveyard to the battlefield tapped. It perpetually gets +1/+1. Activate only if you control a 1/1 creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                // GAP: no ActivationZone::Graveyard; using Hand as placeholder.
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: rejuvenate,
            }),
    )
}

fn rejuvenate(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: No ActivationZone::Graveyard (can't activate from graveyard).
    // GAP: "perpetually gets +1/+1" (Arena-only permanent buff) has no Effect variant.
    // GAP: "activate only if you control a 1/1 creature" condition not enforceable.
    Vec::new()
}
