//! Sparksmith — `{1}{R}` 1/1 Goblin.
//! `{T}: This creature deals X damage to target creature and X damage to you, where X is
//! the number of Goblins on the battlefield.`

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{
    ActivatedAbilityDef, ActivationContext, ActivationCost, ActivationZone,
    CardDefinition, CardRegistry,
};
use arcana_core::state::GameState;
use arcana_core::targets::{ControllerConstraint, TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, PtValue, SubtypeSet, TypeLine};
use arcana_core::script;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sparksmith");
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
                text: "{T}: This creature deals X damage to target creature and X damage to you, where X is the number of Goblins on the battlefield.".into(),
                cost: ActivationCost::tap_only(),
                target_requirements: vec![TargetRequirement::target_creature()],
                is_mana_ability: false,
                is_loyalty_ability: false,
                activation_zone: ActivationZone::Battlefield,
                is_instant_speed: false,
                face_gate: None,
                effect: goblin_ping,
            }),
    )
}

fn goblin_ping(
    state: &GameState,
    ctx: &ActivationContext,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(target) = ctx.targets.targets.first() else {
        return Vec::new();
    };
    let TargetChoice::Object(id) = target else {
        return Vec::new();
    };
    let goblin_filter = script::subtype_filter(reg, "Goblin")
        .controlled_by(ControllerConstraint::Any);
    let x = script::count_matching(state, &goblin_filter, ctx.controller);
    if x == 0 {
        return Vec::new();
    }
    vec![
        Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Object(*id),
            amount: x,
        },
        Effect::DealDamage {
            source: ctx.source,
            target: DamageTarget::Player(ctx.controller),
            amount: x,
        },
    ]
}
