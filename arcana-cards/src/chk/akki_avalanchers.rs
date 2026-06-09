//! Akki Avalanchers — `{R}` 1/1 red Goblin Warrior. "Sacrifice a land: This
//! creature gets +2/+0 until end of turn. Activate only once each turn."
//!
//! GAP: "Sacrifice a land" activation cost (non-self) is not expressible via
//! ActivationCost — sacrifice: true sacrifices the source.

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
    let name = reg.interner_mut().intern("Akki Avalanchers");
    let goblin = reg.interner_mut().intern("Goblin");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice a land: This creature gets +2/+0 until end of turn. Activate only once each turn.".into(),
                cost: ActivationCost {
                    sacrifice: true,
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_two_zero,
            }),
    )
}

fn pump_two_zero(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "Sacrifice a land" (non-self) cost not expressible; sacrifice: true
    // sacrifices the source.
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 0,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
