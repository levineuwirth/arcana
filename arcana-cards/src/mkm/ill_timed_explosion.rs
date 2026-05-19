//! Ill-Timed Explosion — `{2}{U}{R}` sorcery. "Draw two cards. Then you may
//! discard two cards. When you do, Ill-Timed Explosion deals X damage to each
//! creature, where X is the greatest mana value among cards discarded this
//! way."
//!
//! GAP: Optional discard with a conditional triggered damage effect (X =
//! greatest mana value among discarded cards) is not expressible — requires
//! player choice on whether to discard, tracking which specific cards were
//! discarded, computing max mana value, and applying damage to each creature.
//! Best effort: emit DrawCards 2 only; the optional discard rider is omitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ill-Timed Explosion");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw two cards. Then you may discard two cards. When you do, Ill-Timed Explosion deals X damage to each creature, where X is the greatest mana value among cards discarded this way.".into(),
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
    // GAP: optional discard + conditional "when you do" damage to each creature
    // where X = greatest mana value of discarded cards — not expressible.
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
    ]
}
