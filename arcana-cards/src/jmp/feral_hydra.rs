//! Feral Hydra — `{X}{G}` 0/0 Hydra Beast.
//!
//! * "This creature enters with X +1/+1 counters on it." — GAP: the
//!   cast-time X value is not readable from the available ETB accessors,
//!   and there is no enters-with primitive in this surface.
//! * "{3}: Put a +1/+1 counter on this creature. Any player may activate
//!   this ability." — mana activation adding a +1/+1 counter to self. GAP
//!   (partial): "any player may activate" has no controller-override field;
//!   modeled as a normal controller-only activation.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Feral Hydra");
    let hydra = reg.interner_mut().intern("Hydra");
    let beast = reg.interner_mut().intern("Beast");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(hydra);
    subtypes.0.insert(beast);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(0)),
        toughness: Some(PtValue::Fixed(0)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{3}: Put a +1/+1 counter on Feral Hydra.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{3}").unwrap(),
                ..ActivationCost::default()
            },
            target_requirements: vec![],
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: add_counter_to_self,
        }),
    )
}

fn add_counter_to_self(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::AddCounters {
        target: ctx.source,
        kind: CounterKind::PlusOnePlusOne,
        count: 1,
    }]
}
