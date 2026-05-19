//! Suffocating Blast — `{1}{U}{U}{R}` instant, "Counter target spell. Suffocating
//! Blast deals 3 damage to that spell's controller."
//! Two effects: counter the spell + deal 3 damage to its controller.
//! GAP: identifying the spell's controller from the stack — damage applied to
//! the caster of the countered spell; best effort damages the spell's stack
//! entry controller by using the second target as a player target.

use arcana_core::effects::Effect;
use arcana_core::events::DamageTarget;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Suffocating Blast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. Suffocating Blast deals 3 damage to that spell's controller.".into(),
                target_requirements: vec![
                    TargetRequirement {
                        filter: TargetFilter::Spell(ObjectFilter::default()),
                        count: TargetCount::Exactly(1),
                        controller: None,
                    },
                    TargetRequirement::target_player(),
                ],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    let Some(t0) = entry.targets.targets.first() else { return Vec::new(); };
    let Some(t1) = entry.targets.targets.get(1) else { return Vec::new(); };
    let stack_id = match t0 {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    let dt = match t1 {
        TargetChoice::Player(p) => DamageTarget::Player(*p),
        TargetChoice::Object(id) => DamageTarget::Object(*id),
        _ => return Vec::new(),
    };
    vec![
        Effect::Counter { target: stack_id },
        Effect::DealDamage { source: entry.source, target: dt, amount: 3 },
    ]
}
