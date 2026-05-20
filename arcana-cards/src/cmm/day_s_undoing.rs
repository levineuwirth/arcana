//! Day's Undoing — `{2}{U}` sorcery. "Each player shuffles their
//! hand and graveyard into their library, then draws seven cards. If
//! it's your turn, end the turn."
//!
//! GAP: 'shuffle hand+graveyard into library', 'end the turn' are not
//! in the catalog. We approximate the redraw with a per-player draw
//! 7, which is materially incomplete; the spec text is dynamic in
//! that prior-hand size affects the redraw, but here we accept the
//! GAP.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Day's Undoing");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player shuffles their hand and graveyard into their library, then draws seven cards. If it's your turn, end the turn.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, _entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: shuffle-hand+graveyard, end-the-turn not in catalog.
    vec![Effect::Sequence(
        script::all_players(state)
            .into_iter()
            .map(|p| Effect::DrawCards { player: p, count: 7 })
            .collect(),
    )]
}
