//! Molten Influence — `{1}{R}` instant. "Counter target instant or sorcery
//! spell unless its controller has Molten Influence deal 4 damage to them."
//!
//! # GAP: "counter unless controller takes damage" variant
//! The engine only supports `CounterUnlessPays` with a mana cost; there is no
//! variant for "counter unless controller accepts damage instead."
//! Best-effort: emit a hard counter on the instant/sorcery spell.

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
                filter: TargetFilter::Spell(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::INSTANT | TypeLine::SORCERY)),
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
    // GAP: no "counter unless controller takes 4 damage" variant; emitting hard counter
    vec![Effect::Counter { target: *id }]
}
