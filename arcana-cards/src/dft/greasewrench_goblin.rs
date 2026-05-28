//! Greasewrench Goblin — `{R}` 2/1 Goblin Artificer.
//! Exhaust — `{2}{R}: Discard up to two cards, then draw that many cards. Put a +1/+1
//! counter on this creature.`
//! GAP: "Discard up to two cards, then draw that many" — DiscardChoice::ControllerChooses
//! for up to 2; dynamic draw count depends on how many were actually discarded.
//! GAP: Exhaust (activate only once) — no `is_exhaust` field in ActivatedAbilityDef.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Greasewrench Goblin");
    let goblin = reg.interner_mut().intern("Goblin");
    let artificer = reg.interner_mut().intern("Artificer");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    subtypes.0.insert(artificer);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(2)),
        toughness: Some(PtValue::Fixed(1)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_activated_ability(ActivatedAbilityDef {
                text: "Exhaust — {2}{R}: Discard up to two cards, then draw that many cards. Put a +1/+1 counter on this creature.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: loot_and_grow,
            }),
    )
}

fn loot_and_grow(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "discard up to 2 cards, draw that many" — dynamic draw count depends on
    // actual cards discarded; using discard 2 + draw 2 as approximation.
    vec![
        Effect::Discard {
            player: ctx.controller,
            count: 2,
            choice: DiscardChoice::ControllerChooses,
        },
        Effect::DrawCards { player: ctx.controller, count: 2 },
        Effect::AddCounters {
            target: ctx.source,
            kind: CounterKind::PlusOnePlusOne,
            count: 1,
        },
    ]
}
