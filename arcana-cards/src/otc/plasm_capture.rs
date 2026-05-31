//! Plasm Capture — `{G}{G}{U}{U}` instant. "Counter target spell. At
//! the beginning of your next first main phase, add X mana in any
//! combination of colors, where X is that spell's mana value."
//!
//! The counter is expressible via `Effect::Counter`. The delayed,
//! deferred-to-next-main-phase mana production whose amount equals the
//! countered spell's mana value cannot be expressed: there is no
//! delayed-trigger mana primitive, and the countered spell's mana
//! value cannot be captured into a later `Effect::AddMana`.

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
    let name = reg.interner_mut().intern("Plasm Capture");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{G}{G}{U}{U}").expect("valid cost")),
        colors: ColorSet::green() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. At the beginning of your next first main phase, add X mana in any combination of colors, where X is that spell's mana value.".into(),
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
    let stack_id = match target {
        TargetChoice::Object(id) => *id,
        _ => return Vec::new(),
    };
    // GAP: delayed mana at next first main phase equal to the countered
    // spell's mana value (no delayed-trigger mana primitive; cannot
    // capture the countered spell's mana value into a later AddMana).
    vec![Effect::Counter { target: stack_id }]
}
