//! Merchant's Dockhand — `{1}` 1/2 colorless Artifact Creature — Construct.
//! "{3}{U}, {T}, Tap X untapped artifacts you control: Look at the top X cards
//! of your library. Put one of them into your hand and the rest on the bottom
//! in any order."
//!
//! GAP: "Tap X untapped artifacts" (variable non-self tapping) and "look at top
//! X, choose 1 for hand, rest on bottom" not expressible.

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
    let name = reg.interner_mut().intern("Merchant's Dockhand");
    let construct = reg.interner_mut().intern("Construct");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(construct);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{3}{U}, {T}, Tap X untapped artifacts you control: Look at the top X cards of your library. Put one into your hand and the rest on the bottom.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{3}{U}").unwrap(),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: look_and_choose,
            }),
    )
}

fn look_and_choose(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "tap X untapped artifacts" variable cost and "look at top X, choose 1"
    // not expressible.
    Vec::new()
}
