//! Blufferfish — `{4}{U}{U}` 3/6 blue Fish.
//! "{3}{U}: An opponent tells you a statement about themselves and secretly notes
//! whether it's true or false. You guess which it is. If you guessed correctly,
//! this creature gets +2/+0 and can't be blocked this turn."
//! GAP: The "bluff/guess" interactive mini-game mechanic (opponent states something,
//! controller guesses truth value) cannot be expressed with any Effect catalog variant.

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
    let name = reg.interner_mut().intern("Blufferfish");
    let fish = reg.interner_mut().intern("Fish");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(fish);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(6)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}: An opponent tells you a statement about themselves and secretly notes whether it's true or false. You guess which it is. If you guessed correctly, this creature gets +2/+0 and can't be blocked this turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: bluff_effect,
            }),
    )
}

fn bluff_effect(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: The bluff/guess mini-game (opponent declares truth, controller guesses)
    // is not modeled in the Effect catalog. No Effect variant represents interactive
    // guessing or conditional effects based on guess correctness.
    Vec::new()
}
