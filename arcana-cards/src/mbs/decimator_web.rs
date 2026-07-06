//! Decimator Web — `{4}` artifact (Mirrodin Besieged, 2011).
//! "{4}, {T}: Target opponent loses 2 life, gets a poison counter,
//! then mills six cards."
//!
//! The life loss, poison counter, and mill are wired in order; the poison
//! counter on the target player is via Effect::GivePlayerCounters { kind: Poison }.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Decimator Web");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}").expect("valid cost")),
        colors: ColorSet::new(),
        types: TypeLine::ARTIFACT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_activated_ability(
            ActivatedAbilityDef {
                text: "{4}, {T}: Target opponent loses 2 life, gets a \
                       poison counter, then mills six cards."
                    .into(),
                cost: ActivationCost {
                    mana_cost: ManaCost::parse("{4}").expect("valid cost"),
                    tap: true,
                    ..ActivationCost::default()
                },
                // 'Target opponent' — player target (opponent-only
                // restriction is not expressible on a player target).
                target_requirements: vec![TargetRequirement::target_opponent()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: decimate,
            },
        ),
    )
}

fn decimate(
    _state: &GameState,
    ctx: &ActivationContext,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Player(p)) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    // "loses 2 life, gets a poison counter, then mills six cards."
    vec![
        Effect::LoseLife { player: *p, amount: 2 },
        Effect::GivePlayerCounters { player: *p, kind: CounterKind::Poison, count: 1 },
        Effect::Mill { player: *p, count: 6 },
    ]
}
