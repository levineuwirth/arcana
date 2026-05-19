//! Cerebral Download — `{4}{U}` instant, "Surveil X, where X is the number of
//! artifacts you control. Then draw three cards."
//!
//! # GAP: surveil-X-equal-to-artifact-count — no Effect variant for a dynamic
//! Surveil count based on number of artifacts controlled. Draw three is expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cerebral Download");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Surveil X, where X is the number of artifacts you control. Then draw three cards.".into(),
                target_requirements: vec![],
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
    // GAP: surveil-X-equal-to-artifact-count — Effect::Surveil requires a fixed
    // count; dynamic count based on artifacts controlled is not expressible.
    // Emitting only the draw.
    vec![
        Effect::DrawCards { player: entry.controller, count: 3 },
    ]
}
