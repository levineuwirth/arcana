//! Sabertooth Alley Cat — `{1}{R}{R}` 2/1 Cat.
//! This creature attacks each combat if able. (GAP'd — static
//! must-attack restriction, no trigger / activated / keyword form.)
//! {1}{R}: Creatures without defender can't block this creature this
//! turn. (effect GAP'd — no blocker-filtered can't-block restriction;
//! full CantBeBlocked would wrongly exclude defenders too.)

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
    let name = reg.interner_mut().intern("Sabertooth Alley Cat");
    let cat = reg.interner_mut().intern("Cat");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(cat);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    // GAP: "attacks each combat if able" is a static must-attack
    // restriction with no expressible hook.
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{R}: Creatures without defender can't block this creature this turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{R}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: cant_be_blocked_by_nondefenders,
        }),
    )
}

fn cant_be_blocked_by_nondefenders(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "creatures without defender can't block this creature this
    // turn" — no blocker-FILTERED can't-block restriction primitive.
    // Effect::CantBeBlocked is unconditional and would wrongly stop
    // defenders from blocking too.
    Vec::new()
}
