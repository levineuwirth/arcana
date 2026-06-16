//! Urza, Academy Headmaster — `{W}{U}{B}{R}{G}` Legendary Planeswalker —
//! Urza, starting loyalty 6.
//!
//! +1: Head to AskUrza.com and click +1.
//! −1: Head to AskUrza.com and click -1.
//! −6: Head to AskUrza.com and click -6.
//!
//! GAP: all of Urza's abilities are Un-set "out-of-game web service" effects
//!   with no in-engine semantics. Each ability shell is declared with the
//!   correct loyalty cost and returns Vec::new().

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{
    CardId, ColorSet, CounterKind, SubtypeSet, SupertypeSet, TypeLine,
};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urza, Academy Headmaster");
    let urza = reg.interner_mut().intern("Urza");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(urza);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{W}{U}{B}{R}{G}").expect("valid cost")),
        colors: ColorSet::white()
            | ColorSet::blue()
            | ColorSet::black()
            | ColorSet::red()
            | ColorSet::green(),
        types: TypeLine::PLANESWALKER.into(),
        subtypes,
        supertypes: SupertypeSet(SupertypeSet::LEGENDARY),
        loyalty: Some(6),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "+1: Head to AskUrza.com and click +1.".into(),
                cost: ActivationCost {
                    add_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_effect,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-1: Head to AskUrza.com and click -1.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_effect,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "-6: Head to AskUrza.com and click -6.".into(),
                cost: ActivationCost {
                    remove_self_counter: Some((CounterKind::Loyalty, 6)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: true,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: gap_effect,
            }),
    )
}

fn gap_effect(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: Un-set out-of-game web-service ability with no in-engine semantics.
    Vec::new()
}
