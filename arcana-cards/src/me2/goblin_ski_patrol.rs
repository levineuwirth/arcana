//! Goblin Ski Patrol — `{1}{R}` 1/1 Goblin. `{1}{R}: This creature gets
//! +2/+0 and gains flying until end of turn. Its controller sacrifices it
//! at the beginning of the next end step.`
//! GAP: "Activate only once" — per-game activation gate not modeled.
//! GAP: "only if you control a snow Mountain" — snow supertype precondition
//! not modeled in ActivationCost.

use arcana_core::effects::{DelayedAction, DelayedWhen, Effect, KeywordAbility};
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
    let name = reg.interner_mut().intern("Goblin Ski Patrol");
    let goblin = reg.interner_mut().intern("Goblin");
    let mut subtypes = SubtypeSet::default();
    subtypes.0.insert(goblin);
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
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
                text: "{1}{R}: This creature gets +2/+0 and gains flying until end of turn. Its controller sacrifices it at the beginning of the next end step. Activate only once and only if you control a snow Mountain.".into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{1}{R}").unwrap(),
                    ..ActivationCost::default()
                },
                target_requirements: Vec::new(),
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: pump_fly_then_sacrifice,
            }),
    )
}

fn pump_fly_then_sacrifice(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![
        Effect::Pump {
            target: ctx.source,
            power: 2,
            toughness: 0,
            duration: Duration::EndOfTurn,
            keywords: vec![KeywordAbility::Flying],
        },
        Effect::DelayedAction {
            source: ctx.source,
            controller: ctx.controller,
            when: DelayedWhen::NextEndStep,
            action: DelayedAction::Sacrifice,
        },
    ]
}
