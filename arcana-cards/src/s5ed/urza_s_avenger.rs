//! Urza's Avenger — `{6}` 4/4 Artifact Creature — Shapeshifter.
//! `{0}: This creature gets -1/-1 and gains your choice of banding, flying,
//! first strike, or trample until end of turn.`
//!
//! GAP: The "your choice of [keyword A, B, C, or D]" selection mechanic
//! is not expressible in the engine API — there is no modal dispatch for
//! activated abilities. The -1/-1 pump is modeled; the keyword grant is
//! omitted entirely.

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
    let name = reg.interner_mut().intern("Urza's Avenger");
    let shapeshifter = reg.interner_mut().intern("Shapeshifter");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(shapeshifter);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}").expect("valid cost")),
        colors: ColorSet::colorless(),
        types: TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(4)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{0}: This creature gets -1/-1 and gains your choice of banding, flying, first strike, or trample until end of turn.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{0}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_minus_and_keyword,
            }),
    )
}

fn pump_minus_and_keyword(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "your choice of banding, flying, first strike, or trample" —
    // modal keyword selection for activated abilities is not supported.
    // Only the -1/-1 power/toughness reduction is modeled.
    vec![Effect::Pump {
        target: ctx.source,
        power: -1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
