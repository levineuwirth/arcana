//! Deadshot — `{3}{R}` sorcery. "Tap target creature. It deals damage
//! equal to its power to another target creature."

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Deadshot");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Tap target creature. It deals damage equal to its power to another target creature.".into(),
            target_requirements: vec![
                TargetRequirement::target_creature(),
                TargetRequirement::target_creature(),
            ],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let targets = &entry.targets.targets;
    let Some(TargetChoice::Object(t0)) = targets.first() else { return Vec::new(); };
    let Some(TargetChoice::Object(t1)) = targets.get(1) else { return Vec::new(); };
    let power = script::power_of(state, *t0).max(0) as u32;
    vec![
        Effect::Tap { target: *t0 },
        Effect::DealDamage {
            source: *t0,
            target: DamageTarget::Object(*t1),
            amount: power,
        },
    ]
}
