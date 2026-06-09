//! Cutthroat Centurion — `{2}{B}` 2/2 black Artifact Creature — Phyrexian
//! Warrior. "Sacrifice another artifact or creature: This creature gets +2/+2
//! until end of turn. Activate only once each turn."
//!
//! GAP: "Sacrifice another artifact or creature" (non-self) activation cost
//! not expressible via ActivationCost.

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
    let name = reg.interner_mut().intern("Cutthroat Centurion");
    let phyrexian = reg.interner_mut().intern("Phyrexian");
    let warrior = reg.interner_mut().intern("Warrior");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(phyrexian);
    subtypes.0.insert(warrior);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE).into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice another artifact or creature: This creature gets +2/+2 until end of turn. Activate only once each turn.".into(),
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
                effect: pump_two_two,
            }),
    )
}

fn pump_two_two(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "sacrifice another artifact or creature" (non-self) not expressible.
    vec![Effect::Pump {
        target: ctx.source,
        power: 2,
        toughness: 2,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
