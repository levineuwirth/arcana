//! Induce Paranoia — `{2}{U}{U}` instant. "Counter target spell. If
//! {B} was spent to cast this spell, that spell's controller mills X
//! cards, where X is the spell's mana value."
//!
//! The counter is expressible. The conditional rider (detecting whether
//! {B} was spent to cast this spell, then milling the countered spell's
//! controller for that spell's mana value) is not: there is no helper
//! to inspect mana spent on this spell, to read the countered spell's
//! controller, or to read a stack object's mana value. The mill rider
//! is left as a GAP rather than hardcoding a count.

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
    let name = reg.interner_mut().intern("Induce Paranoia");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target spell. If {B} was spent to cast this spell, that spell's controller mills X cards, where X is the spell's mana value.".into(),
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
    // GAP: cannot detect whether {B} was spent to cast this spell, nor
    // read the countered spell's controller / mana value to mill X.
    vec![Effect::Counter { target: stack_id }]
}
