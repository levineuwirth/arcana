//! Time Reversal — `{3}{U}{U}` sorcery. "Each player shuffles their hand
//! and graveyard into their library, then draws seven cards. Exile Time
//! Reversal."
//!
//! GAP: no Effect to shuffle hand+graveyard into library, nor to self-exile
//! the spell. The per-player draw seven is emitted as best-effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Time Reversal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player shuffles their hand and graveyard into their library, then draws seven cards. Exile Time Reversal.".into(),
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
    // GAP: no shuffle-hand-and-graveyard-into-library effect, no self-exile.
    vec![Effect::Sequence(
        script::all_players(state)
            .into_iter()
            .map(|p| Effect::DrawCards { player: p, count: 7 })
            .collect(),
    )]
}
