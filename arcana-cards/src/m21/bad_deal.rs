//! Bad Deal — `{4}{B}{B}` Sorcery. "Each player draws two cards, then
//! discards two cards. Each player loses 2 life."
//!
//! # Implementation note
//! DrawCards, Discard, and LoseLife are expressible. "Each player"
//! requires iterating over all player IDs. We can express the caster's
//! effects directly; per-opponent iteration is approximated using the
//! same effects on entry.controller — the "each player" aspect is a
//! partial gap.
//!
//! # GAP
//! No Effect for iterating effects over all players; only caster's
//! effects are expressed here.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Bad Deal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player draws two cards, then discards two cards. Each player loses 2 life.".into(),
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
    // GAP: "each player" iteration not expressible; only controller effects expressed
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::Discard { player: entry.controller, count: 2, choice: DiscardChoice::ControllerChooses },
        Effect::LoseLife { player: entry.controller, amount: 2 },
    ]
}
