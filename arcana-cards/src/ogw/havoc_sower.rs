//! Havoc Sower — `{3}{B}` 3/3 Eldrazi Drone, Devoid (colorless).
//!
//! * Devoid — "this card has no color"; reflected by `colors:
//!   ColorSet::colorless()`. Devoid is not in the usable KeywordAbility surface,
//!   so it is not listed in `keywords`.
//! * "{1}{C}: This creature gets +2/+1 until end of turn." — a mana activated
//!   ability pumping the source +2/+1.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Havoc Sower");
    let eldrazi = reg.interner_mut().intern("Eldrazi");
    let drone = reg.interner_mut().intern("Drone");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(eldrazi);
    subtypes.0.insert(drone);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(ActivatedAbilityDef {
            text: "{1}{C}: This creature gets +2/+1 until end of turn.".into(),
            cost: ActivationCost {
                mana_cost: ManaCost::parse("{1}{C}").expect("valid cost"),
                ..ActivationCost::default()
            },
            target_requirements: Vec::new(),
            is_mana_ability: false,
            is_loyalty_ability: false,
            activation_zone: ActivationZone::Battlefield,
            is_instant_speed: false,
            face_gate: None,
            effect: pump_self,
        }),
    )
}

fn pump_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
