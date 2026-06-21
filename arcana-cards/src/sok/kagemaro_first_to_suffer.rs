//! Kagemaro, First to Suffer — `{3}{B}{B}` */* Legendary Demon Spirit.
//! Kagemaro's power and toughness are each equal to the number of cards
//! in your hand (CDA — recorded as */* via PtValue::Star; the static is
//! not expressible via the effect surface — GAP).
//! {B}, Sacrifice Kagemaro: All creatures get -X/-X until end of turn,
//! where X is the number of cards in your hand.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::script;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, SupertypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kagemaro, First to Suffer");
    let demon = reg.interner_mut().intern("Demon");
    let spirit = reg.interner_mut().intern("Spirit");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(demon);
    subtypes.0.insert(spirit);

    // GAP: CDA "power and toughness equal to cards in your hand" — bones
    // recorded as */* (PtValue::Star).
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        power: Some(PtValue::Star),
        toughness: Some(PtValue::Star),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{B}, Sacrifice Kagemaro: All creatures get -X/-X until end of turn, where X is the number of cards in your hand.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{B}").expect("valid cost"),
                sacrifice: true,
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: all_creatures_minus_x,
        }),
    )
}

fn all_creatures_minus_x(
    state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let x = script::hand_size(state, ctx.controller) as i32;
    let ids = script::ids_matching(state, &ObjectFilter::creature(), ctx.controller);
    vec![Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::Pump {
            target: NULL_OBJECT_ID,
            power: -x,
            toughness: -x,
            duration: Duration::EndOfTurn,
            keywords: vec![],
        }),
    }]
}
