//! Self-Destruct — `{1}{R}` instant. "Target creature you control
//! deals X damage to any other target and X damage to itself, where X
//! is its power."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectOrPlayer, TargetChoice, TargetRequirement,
};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Self-Destruct");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Target creature you control deals X damage to any other target and X damage to itself, where X is its power.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::any_target(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(TargetChoice::Object(creature)) = entry.targets.targets.first() else {
        return Vec::new();
    };
    let Some(other) = entry.targets.targets.get(1) else { return Vec::new(); };
    let x = script::power_of(state, *creature).max(0) as u32;
    let other_dt = match other {
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Object(id)) => {
            DamageTarget::Object(*id)
        }
        TargetChoice::ObjectOrPlayer(ObjectOrPlayer::Player(p)) => {
            DamageTarget::Player(*p)
        }
    };
    vec![
        Effect::DealDamage { source: *creature, target: other_dt, amount: x },
        Effect::DealDamage {
            source: *creature,
            target: DamageTarget::Object(*creature),
            amount: x,
        },
    ]
}
