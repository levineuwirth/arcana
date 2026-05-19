//! Electrosiphon — `{U}{U}{R}` instant, "Counter target spell. You get an
//! amount of {E} (energy counters) equal to its mana value."
//!
//! # GAP
//! * GAP: energy counter mechanic (no AddEnergyCounters Effect variant)
//! * GAP: amount equal to target spell's mana value (dynamic value from stack object)

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Electrosiphon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. You get an amount of {E} equal to its mana value.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(ObjectFilter::default()),
                    count: TargetCount::Exactly(1),
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
    let Some(target) = entry.targets.targets.first() else { return Vec::new(); };
    let TargetChoice::Object(stack_id) = target else { return Vec::new(); };
    vec![
        Effect::Counter { target: *stack_id },
        // GAP: energy counter mechanic (no AddEnergyCounters Effect variant)
        // GAP: amount equal to target spell's mana value (dynamic value from stack object)
    ]
}
