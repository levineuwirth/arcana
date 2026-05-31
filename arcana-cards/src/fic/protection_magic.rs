//! Protection Magic — `{1}{W}` instant. "Put a shield counter on each
//! of up to three target creatures." (A shield counter is removed
//! instead of the creature being dealt damage or destroyed.)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, CounterKind, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Protection Magic");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Put a shield counter on each of up to three target creatures.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Creature,
                count: TargetCount::UpTo(3),
                controller: None,
            }],
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
    let mut effects = Vec::new();
    for target in &entry.targets.targets {
        if let TargetChoice::Object(id) = target {
            effects.push(Effect::AddCounters {
                target: *id,
                kind: CounterKind::Shield,
                count: 1,
            });
        }
    }
    effects
}
