//! Selvala's Charge — `{4}{G}` sorcery. "Parley — Each player reveals
//! the top card of their library. For each nonland card revealed this
//! way, you create a 3/3 green Elephant creature token. Then each
//! player draws a card."
//!
//! GAP: the Parley reveal and the token count keyed off "nonland cards
//! revealed this way" cannot be computed from the script helpers (no
//! library-reveal Effect, no per-reveal accessor). The final "each
//! player draws a card" clause is modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Selvala's Charge");
    let _elephant = reg.interner_mut().intern("Elephant");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Parley — Each player reveals the top card of their library. For each nonland card revealed this way, you create a 3/3 green Elephant creature token. Then each player draws a card.".into(),
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
    // GAP: Parley reveal + token-per-nonland-revealed not computable.
    vec![Effect::Sequence(
        script::all_players(state)
            .into_iter()
            .map(|p| Effect::DrawCards { player: p, count: 1 })
            .collect(),
    )]
}
