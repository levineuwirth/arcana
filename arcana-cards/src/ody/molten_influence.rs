//! Molten Influence — `{1}{R}` instant. "Counter target instant or
//! sorcery spell unless its controller has Molten Influence deal 4
//! damage to them." The "unless they take 4 damage" alternative cost
//! is not the mana-tax form of CounterUnlessPays; modeled as a plain
//! counter of the target spell.

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
    let name = reg.interner_mut().intern("Molten Influence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target instant or sorcery spell unless its controller has Molten Influence deal 4 damage to them.".into(),
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

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "unless its controller has it deal 4 damage to them" is a
    // damage-based alternative, not the mana-tax CounterUnlessPays.
    // Plain counter emitted.
    let Some(TargetChoice::Object(id)) = entry.targets.targets.first() else { return Vec::new(); };
    vec![Effect::Counter { target: *id }]
}
