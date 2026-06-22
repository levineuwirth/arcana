//! Ghost-Lit Redeemer — `{W}` 1/1 white Spirit.
//!
//! Oracle:
//! * {W}, {T}: You gain 2 life.
//! * Channel — {1}{W}, Discard this card: You gain 4 life.
//!
//! The battlefield pinger-style life gain and the hand-activated Channel
//! ability are both wired. Channel uses `discard_self` as its cost and
//! activates from the hand zone.

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
    let name = reg.interner_mut().intern("Ghost-Lit Redeemer");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(spirit);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{W}, {T}: You gain 2 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{W}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_two,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Channel — {1}{W}, Discard this card: You gain 4 life.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{W}").expect("valid cost"),
                    discard_self: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Hand,
                is_instant_speed: false,
                face_gate: None,
                effect: gain_four,
            }),
    )
}

fn gain_two(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: ctx.controller, amount: 2 }]
}

fn gain_four(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::GainLife { player: ctx.controller, amount: 4 }]
}
