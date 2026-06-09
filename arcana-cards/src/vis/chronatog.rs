//! Chronatog — `{1}{U}` 1/2 Atog.
//! `{0}: This creature gets +3/+3 until end of turn. You skip your next turn. Activate
//! only once each turn.`
//! GAP: "skip your next turn" — no Effect for skipping a turn.

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Chronatog");
    let atog = reg.interner_mut().intern("Atog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(atog);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{0}: This creature gets +3/+3 until end of turn. You skip your next turn. Activate only once each turn.".into(),
                cost: ActivationCost {
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_and_skip,
            }),
    )
}

fn pump_and_skip(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "you skip your next turn" — no turn-skip Effect
    vec![Effect::Pump {
        target: ctx.source,
        power: 3,
        toughness: 3,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
