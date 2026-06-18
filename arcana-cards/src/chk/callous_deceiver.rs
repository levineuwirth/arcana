//! Callous Deceiver — `{2}{U}` 1/3 Spirit.
//! "{1}: Look at the top card of your library." (GAP — purely informational,
//! no "look at top card" effect primitive.)
//! "{2}: Reveal the top card of your library. If it's a land card, this
//! creature gets +1/+0 and gains flying until end of turn. Activate only once
//! each turn." (GAP — reveal-and-test-top-card conditional is not expressible;
//! the once-per-turn limit is captured on the cost.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Callous Deceiver");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: Look at the top card of your library.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: look_top,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}: Reveal the top card of your library. If it's a land card, this creature gets +1/+0 and gains flying until end of turn. Activate only once each turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}").expect("valid cost"),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: reveal_top_land_pump,
            }),
    )
}

fn look_top(_state: &GameState, _ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "look at the top card of your library" — informational, no effect primitive.
    Vec::new()
}

fn reveal_top_land_pump(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: reveal top card and conditionally pump/grant flying if it's a land —
    // no reveal-and-test-top-card conditional effect.
    Vec::new()
}
