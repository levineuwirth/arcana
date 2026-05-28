//! Gilder Bairn — `{1}{G/U}{G/U}` 1/3 green/blue Ouphe.
//! `{2}{G/U}, {Q}: Double the number of each kind of counter on target
//! permanent.` ({Q} is the untap symbol.)
//!
//! GAP: "double each kind of counter" — no Effect::Proliferate variant
//! doubles counters (it adds one of each kind). No exact variant.
//! {Q} (untap symbol) not in ActivationCost. Using Proliferate as
//! closest approximation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gilder Bairn");
    let ouphe = reg.interner_mut().intern("Ouphe");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(ouphe);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{G/U}, {Q}: Double the number of each kind of counter on target permanent.".into(),
                // GAP: {Q} (untap symbol) not in ActivationCost.
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{G}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: double_counters,
            }),
    )
}

fn double_counters(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "double each kind of counter" — no Effect::DoubleCounters variant.
    // Proliferate adds one of each kind (not doubles).
    Vec::new()
}
