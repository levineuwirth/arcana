//! Vampire Hounds — `{2}{B}` 2/2 Creature — Vampire Dog.
//! Discard a creature card: This creature gets +2/+2 until end of turn.
//! "Discard a creature card" cost modeled via `discard_other` (creature
//! filter).

use arcana_core::effects::Effect;
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition, CardRegistry};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Vampire Hounds");
    let vampire = reg.interner_mut().intern("Vampire");
    let dog = reg.interner_mut().intern("Dog");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(vampire);
    subtypes.0.insert(dog);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Discard a creature card: This creature gets +2/+2 until end of turn.".into(),
                cost: ActivationCost {
                    discard_other: Some(arcana_core::targets::ObjectFilter {
                        types: Some(TypeLine::CREATURE.into()),
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
