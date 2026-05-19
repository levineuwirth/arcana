//! Hypothesizzle — `{3}{U}{R}` Instant. "Draw two cards. Then you may
//! discard a nonland card. When you do, Hypothesizzle deals 4 damage
//! to target creature."
//!
//! # Implementation note
//! DrawCards 2 is expressible. The optional discard-then-deal-damage
//! chain (conditional on discarding a nonland) is not expressible
//! without a player-choice conditional not in the Effect catalog.
//!
//! # GAP
//! Optional discard-nonland → deal 4 damage chain not expressible
//! (no player-choice conditional in Effect catalog).

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hypothesizzle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw two cards. Then you may discard a nonland card. When you do, Hypothesizzle deals 4 damage to target creature.".into(),
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
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        // GAP: optional discard-nonland + conditional damage chain not expressible
    ]
}
