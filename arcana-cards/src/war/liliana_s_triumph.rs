//! Liliana's Triumph — `{1}{B}` instant. "Each opponent sacrifices a creature
//! of their choice. If you control a Liliana planeswalker, each opponent also
//! discards a card."
//!
//! GAP: the "if you control a Liliana planeswalker" conditional discard is not
//! expressible (no way to test control of a planeswalker by name). Only the
//! unconditional each-opponent sacrifice is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Liliana's Triumph");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each opponent sacrifices a creature of their choice. If you control a Liliana planeswalker, each opponent also discards a card.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    vec![Effect::Sequence(
        script::opponents(state, entry.controller)
            .into_iter()
            .map(|p| Effect::Sacrifice {
                player: p,
                filter: ObjectFilter::creature(),
                count: 1,
            })
            .collect(),
    )]
}
