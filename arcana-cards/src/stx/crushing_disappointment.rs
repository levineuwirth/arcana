//! Crushing Disappointment — `{3}{B}` instant, "Each player loses 2 life.
//! You draw two cards."
//!
//! # Note
//! "Each player" modeled as two LoseLife effects (one per player). The engine
//! does not expose an opponent PlayerId at resolution; using
//! `entry.controller` for self and noting the opponent effect as a GAP.
//!
//! # GAP
//! No way to enumerate all PlayerIds at resolve time; opponent LoseLife is not
//! expressible. Self LoseLife + DrawCards are emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Crushing Disappointment");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player loses 2 life. You draw two cards.".into(),
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
    // GAP: opponent PlayerId not available at resolve time; emitting self only
    vec![
        Effect::LoseLife { player: entry.controller, amount: 2 },
        Effect::DrawCards { player: entry.controller, count: 2 },
    ]
}
