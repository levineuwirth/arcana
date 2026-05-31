//! Hurl into History — `{3}{U}{U}` instant. "Counter target artifact
//! or creature spell. Discover X, where X is that spell's mana value."
//!
//! The counter half is fully expressible via `Effect::Counter` against
//! a `TargetFilter::Spell` restricted to artifact-or-creature spells.
//! The Discover rider (impulse-cast/hand from the top of the library
//! down to a mana-value gate equal to the countered spell's mana value)
//! has no engine primitive yet, so it is GAP-ed.

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
    let name = reg.interner_mut().intern("Hurl into History");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Counter target artifact or creature spell. Discover X, where X is that spell's mana value.".into(),
            target_requirements: vec![TargetRequirement {
                filter: TargetFilter::Spell(
                    ObjectFilter::new()
                        .with_types_any(TypeLine(TypeLine::ARTIFACT | TypeLine::CREATURE)),
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
    // GAP: Discover X (impulse cast / put-to-hand a nonland card with mana
    // value <= the countered spell's mana value) — no Discover effect
    // primitive in the catalog. Counter half is emitted faithfully.
    vec![Effect::Counter { target: stack_id }]
}
