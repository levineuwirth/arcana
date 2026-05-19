//! Concerted Defense — `{U}` instant.
//! "Counter target noncreature spell unless its controller pays {1} plus an additional {1}
//! for each creature in your party."
//! GAP: party count (up to one each of Cleric, Rogue, Warrior, Wizard) is not available via
//! the scripting API; CounterUnlessPays requires a fixed cost, not a dynamic one.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Concerted Defense");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target noncreature spell unless its controller pays {1} plus an additional {1} for each creature in your party.".into(),
                target_requirements: vec![TargetRequirement {
                    filter: TargetFilter::Spell(
                        ObjectFilter::new().without_types(TypeLine::CREATURE.into())
                    ),
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
    let TargetChoice::Object(id) = target else { return Vec::new(); };
    // GAP: dynamic tax cost based on party count; using fixed {1} as minimum
    vec![Effect::CounterUnlessPays {
        target: *id,
        cost: ManaCost::parse("{1}").expect("valid cost"),
    }]
}
