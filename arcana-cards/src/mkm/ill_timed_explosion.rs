//! Ill-Timed Explosion — `{2}{U}{R}` sorcery. "Draw two cards. Then
//! you may discard two cards. When you do, Ill-Timed Explosion deals X
//! damage to each creature, where X is the greatest mana value among
//! cards discarded this way."
//!
//! The opening "Draw two cards" is a plain `Effect::DrawCards`. The
//! follow-up "you may discard two cards. When you do, ~ deals X damage
//! to each creature, where X is the greatest mana value among cards
//! discarded this way" is a reflexive ("when you do") trigger whose
//! damage amount is the greatest mana value of the cards discarded that
//! way. There is no script helper that reports the mana values of cards
//! discarded by a specific effect, and no reflexive-discard-then-damage
//! primitive, so that whole clause is a GAP.

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
        colors: ColorSet::red() | ColorSet::blue(),
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
    // GAP: reflexive "you may discard two cards. When you do, deal X
    // damage to each creature, where X is the greatest mana value among
    // cards discarded this way" — no primitive ties an optional discard
    // to a follow-up damage amount derived from the discarded cards'
    // mana values. Only the leading draw is expressible.
    vec![Effect::DrawCards { player: entry.controller, count: 2 }]
}
