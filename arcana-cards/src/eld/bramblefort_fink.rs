//! Bramblefort Fink — `{1}{G}` 2/2 Ouphe.
//! `{8}:` This creature has base power and toughness 10/10 until end of turn.
//! Activate only if you control an Oko planeswalker.
//! "Activate only if you control an Oko planeswalker" modeled via
//! `activation_condition` + `conditions::you_control_subtype`.

use arcana_core::conditions;
use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, ObjectId};
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PlayerId, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bramblefort Fink");
    let ouphe = reg.interner_mut().intern("Ouphe");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ouphe);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{8}: This creature has base power and toughness 10/10 until end of turn. Activate only if you control an Oko planeswalker.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{8}").unwrap(),
                    activation_condition: Some(precond_oko),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: set_10_10,
            }),
    )
}

fn set_10_10(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::SetBasePT {
        target: ctx.source,
        power: 10,
        toughness: 10,
        duration: Duration::EndOfTurn,
    }]
}

fn precond_oko(state: &GameState, _source: ObjectId, you: PlayerId, reg: &CardRegistry) -> bool {
    conditions::you_control_subtype(state, reg, you, "Oko")
}
