//! Deadhead — `{3}{B}` 3/3 Zombie.
//! `{0}:` Return this card from your graveyard to the battlefield. Activate
//! only if an opponent isn't touching their hand (of cards).
//! GAP: "activate only if an opponent isn't touching their hand" — physical
//! action precondition not expressible. Graveyard-activation zone supported
//! but the precondition condition is not. Emitting the cost+zone only.

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
    let name = reg.interner_mut().intern("Deadhead");
    let zombie = reg.interner_mut().intern("Zombie");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(zombie);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
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
                text: "{0}: Return this card from your graveyard to the battlefield. Activate only if an opponent isn't touching their hand (of cards).".into(),
                cost: ActivationCost::default(), // {0} cost
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: return_from_gy,
            }),
    )
}

fn return_from_gy(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "return this card from your graveyard" requires graveyard zone
    // activation and a self-reanimate Effect; physical precondition
    // "opponent isn't touching their hand" is not modelable.
    Vec::new()
}
