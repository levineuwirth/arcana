//! Loch Mare — `{1}{U}` 4/5 Horse Serpent.
//!
//! Oracle:
//! * This creature enters with three -1/-1 counters on it.
//! * {1}{U}, Remove a counter from this creature: Draw a card.
//! * {2}{U}, Remove two counters from this creature: Tap target
//!   creature. Put a stun counter on it.
//!
//! The only counters it enters with are -1/-1, so "remove a counter"
//! is modeled as removing MinusOneMinusOne counters.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry, EntersWithSpec,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, PtValue, SubtypeSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Loch Mare");
    let horse = reg.interner_mut().intern("Horse");
    let serpent = reg.interner_mut().intern("Serpent");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(horse);
    subtypes.0.insert(serpent);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::CREATURE.into(),
        subtypes,
        power: Some(PtValue::Fixed(4)),
        toughness: Some(PtValue::Fixed(5)),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_enters_with(EntersWithSpec::Counters {
                kind: CounterKind::MinusOneMinusOne,
                count: 3,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{1}{U}, Remove a counter from this creature: Draw a card.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{U}").unwrap(),
                    remove_self_counter: Some((CounterKind::MinusOneMinusOne, 1)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: draw_a_card,
            })
            .with_activated_ability(ActivatedAbilityDef {
                text: "{2}{U}, Remove two counters from this creature: Tap target \
                       creature. Put a stun counter on it.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{2}{U}").unwrap(),
                    remove_self_counter: Some((CounterKind::MinusOneMinusOne, 2)),
                    ..ActivationCost::default()
                },
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: tap_and_stun,
            }),
    )
}

fn draw_a_card(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::DrawCards { player: ctx.controller, count: 1 }]
}

fn tap_and_stun(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    vec![Effect::Sequence(vec![
        Effect::Tap { target: *id },
        Effect::AddCounters {
            target: *id,
            kind: CounterKind::Stun,
            count: 1,
        },
    ])]
}
