//! Rousing of Souls — `{2}{W}` sorcery. "Parley — Each player reveals
//! the top card of their library. For each nonland card revealed this
//! way, you create a 1/1 white Spirit creature token with flying.
//! Then each player draws a card."
//!
//! The reveal-and-count and the dynamically-scaled token creation are
//! not modeled (no top-card reveal/count helper) — GAP. The trailing
//! "each player draws a card" is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rousing of Souls");
    let _sp = reg.interner_mut().intern("Spirit");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Parley — Each player reveals the top card of their library. For each nonland card revealed this way, you create a 1/1 white Spirit creature token with flying. Then each player draws a card.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    _entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: top-card reveal/count and the dynamically-scaled Spirit
    // token creation are not modeled. The each-player draw is emitted.
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::DrawCards { player: p, count: 1 })
        .collect()
}
