//! Concerted Defense — `{U}` instant. "Counter target noncreature
//! spell unless its controller pays {1} plus an additional {1} for each
//! creature in your party."
//!
//! The base soft-counter ("unless its controller pays {1}") is
//! expressible via `Effect::CounterUnlessPays`. The "plus an additional
//! {1} for each creature in your party" scaling depends on the party
//! count (up to one each of Cleric, Rogue, Warrior, Wizard), which no
//! script:: helper computes and which `CounterUnlessPays` takes as a
//! fixed `ManaCost`. Emitting the {1} floor and GAP-ing the party
//! scaling.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{
    ObjectFilter, TargetChoice, TargetCount, TargetFilter, TargetRequirement,
};
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
                        ObjectFilter::new().without_types(TypeLine::CREATURE.into()),
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
    let stack_id = match target {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    // GAP: the "plus an additional {1} for each creature in your party"
    // scaling cannot be computed (no party-count helper) and
    // CounterUnlessPays takes a fixed ManaCost. Emitting the {1} floor.
    vec![Effect::CounterUnlessPays {
        target: stack_id,
        cost: ManaCost::parse("{1}").expect("valid cost"),
    }]
}
