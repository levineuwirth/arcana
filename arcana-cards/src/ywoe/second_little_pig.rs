//! Second Little Pig — `{1}{W/B}` 2/3 white/black Boar.
//! "{2}{W/B}{W/B}: Second Little Pig perpetually becomes a Boar Spirit with base power
//! and toughness 4/4 and gains flying. Activate only if Second Little Pig isn't a Spirit."
//! GAP: "perpetually becomes" — Arena-only mechanic not in catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Second Little Pig");
    let boar = reg.interner_mut().intern("Boar");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(boar);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W/B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(3)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{W/B}{W/B}: Second Little Pig perpetually becomes a Boar Spirit with base power and toughness 4/4 and gains flying.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{W/B}{W/B}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: perpetually_transform,
            }),
    )
}

fn perpetually_transform(
    _state: &GameState,
    _ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "perpetually becomes" — Arena-only mechanic for cross-zone type/PT changes.
    Vec::new()
}
