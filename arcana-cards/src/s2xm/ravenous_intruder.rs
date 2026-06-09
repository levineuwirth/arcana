//! Ravenous Intruder — `{1}{R}` 1/2 Creature — Gremlin.
//! Sacrifice an artifact: This creature gets +2/+2 until end of turn.
//! "Sacrifice an artifact" cost modeled via `sacrifice_other` (artifact filter).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ravenous Intruder");
    let gremlin = reg.interner_mut().intern("Gremlin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(gremlin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(1)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice an artifact: This creature gets +2/+2 until end of turn.".into(),
                cost: ActivationCost {
                    sacrifice_other: Some(arcana_core::targets::ObjectFilter {
                        types: Some(TypeLine::ARTIFACT.into()),
                        ..Default::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump,
            }),
    )
}

fn pump(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump { target: ctx.source, power: 2, toughness: 2, duration: Duration::EndOfTurn, keywords: vec![] }]
}
