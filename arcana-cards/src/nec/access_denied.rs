//! Access Denied — `{3}{U}{U}` instant. "Counter target spell. Create
//! X 1/1 colorless Thopter artifact creature tokens with flying, where
//! X is that spell's mana value."
//!
//! The counter is expressible. The token rider is not: X is the
//! countered spell's mana value, and there is no script helper to read
//! a stack object's mana value at resolution. Per the dynamic-amount
//! rule, the token creation is left as a GAP rather than minting a
//! hardcoded count.

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
    let name = reg.interner_mut().intern("Access Denied");
    let _thopter = reg.interner_mut().intern("Thopter");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target spell. Create X 1/1 colorless Thopter artifact creature tokens with flying, where X is that spell's mana value.".into(),
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
    // GAP: X is the countered spell's mana value; no script helper reads
    // a stack object's mana value, so the X Thopter tokens are omitted
    // rather than minted with a hardcoded count.
    vec![Effect::Counter { target: stack_id }]
}
