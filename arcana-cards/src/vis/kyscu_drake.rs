//! Kyscu Drake — `{3}{G}` 2/2 green Drake.
//!
//! Flying.
//! {G}: This creature gets +0/+1 until end of turn. Activate only once each turn.
//! Sacrifice this creature and a creature named Spitting Drake: Search
//! your library for a card named Viashivan Dragon, put that card onto
//! the battlefield, then shuffle.
//!
//! Flying (base characteristic) plus two activated abilities: a
//! once-per-turn mana pump on itself, and a sacrifice-pair tutor that
//! fetches Viashivan Dragon onto the battlefield.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::layers::Duration;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone, CardDefinition,
    CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Kyscu Drake");
    let drake = reg.interner_mut().intern("Drake");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(drake);

    // Named-creature sacrifice cost ("a creature named Spitting Drake").
    let spitting_drake = reg.interner_mut().intern("Spitting Drake");

    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(2)),
        keywords: vec![KeywordAbility::Flying],
        ..Default::default()
    };

    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "{G}: This creature gets +0/+1 until end of turn. Activate only once each turn."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{G}").expect("valid cost"),
                    once_per_turn: true,
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_self,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "Sacrifice this creature and a creature named Spitting Drake: Search your library for a card named Viashivan Dragon, put that card onto the battlefield, then shuffle."
                    .into(),
                cost: ActivationCost {
                    sacrifice: true,
                    sacrifice_other: Some(ObjectFilter {
                        name: Some(spitting_drake),
                        ..ObjectFilter::default()
                    }),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: fetch_viashivan,
            }),
    )
}

/// {G}: this creature gets +0/+1 until end of turn.
fn pump_self(_state: &GameState, ctx: &ActivationContext, _reg: &CardRegistry) -> Vec<Effect> {
    vec![Effect::Pump {
        target: ctx.source,
        power: 0,
        toughness: 1,
        duration: Duration::EndOfTurn,
        keywords: vec![],
    }]
}

/// Sacrifice pair: search library for Viashivan Dragon onto the
/// battlefield, then shuffle (shuffle is automatic).
fn fetch_viashivan(_state: &GameState, ctx: &ActivationContext, reg: &CardRegistry) -> Vec<Effect> {
    let nm = reg.interner().lookup("Viashivan Dragon");
    vec![Effect::TutorToBattlefield {
        player: ctx.controller,
        filter: ObjectFilter {
            name: nm,
            ..ObjectFilter::default()
        },
        tapped: false,
    }]
}
