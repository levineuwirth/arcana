//! Spell Swindle — `{3}{U}{U}` instant, "Counter target spell. Create
//! X Treasure tokens, where X is that spell's mana value."
//!
//! The counter is fully expressible; the Treasure rider's count is
//! dynamic on the countered spell's mana value, which is not derivable
//! from the permitted `script::*` helpers (they read battlefield /
//! zone counts, not the targeted spell's CMC), so it is GAP-ed.

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
    let name = reg.interner_mut().intern("Spell Swindle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Counter target spell. Create X Treasure tokens, where X is that spell's mana value.".into(),
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
    // GAP: cannot compute X = the countered spell's mana value with the
    // permitted script:: helpers, so the dynamic Treasure rider is omitted
    // rather than emitted with a wrong literal count.
    vec![Effect::Counter { target: stack_id }]
}
