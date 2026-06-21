//! Flailing Manticore — `{3}{R}` 3/3 Creature — Manticore.
//!
//! Oracle:
//! * Flying, first strike.
//! * {1}: This creature gets +1/+1 until end of turn. Any player may activate
//!   this ability.
//! * {1}: This creature gets -1/-1 until end of turn. Any player may activate
//!   this ability.
//!
//! Decomposition: Flying + First strike → `keywords`; the two pump activations
//! → two `ActivatedAbilityDef`s. The "any player may activate" rider is a
//! fidelity GAP (activated abilities default to the controller); the pump
//! effects themselves are modeled faithfully.

use arcana_core::effects::{Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Flailing Manticore");
    let manticore = reg.interner_mut().intern("Manticore");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(manticore);

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(3)),
        toughness: Some(PtValue::Fixed(3)),
        keywords: vec![KeywordAbility::Flying, KeywordAbility::FirstStrike],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                // GAP fidelity: "Any player may activate this ability" — the
                // open-activation rider isn't modeled; the controller activates.
                text: "{1}: This creature gets +1/+1 until end of turn. Any player may \
                       activate this ability."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_plus,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}: This creature gets -1/-1 until end of turn. Any player may \
                       activate this ability."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}").expect("valid cost"),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_minus,
            }),
    )
}

fn pump_plus(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 1,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

fn pump_minus(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: -1,
        toughness: -1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}
