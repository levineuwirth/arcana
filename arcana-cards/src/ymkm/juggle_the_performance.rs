//! Juggle the Performance — `{1}{B}{R}` sorcery.
//! "Each player discards their hand, then conjures a duplicate of each
//! of seven random cards from the library of the player to their right
//! into their hand. The duplicates perpetually gain 'Mana of any type
//! can be spent to cast this spell.'"
//!
//! The "each player discards their hand" clause is expressible (one
//! `Effect::Discard` per player, count = that player's current hand
//! size). The conjure clause — minting duplicate cards out of another
//! player's library and granting them a perpetual cost-modifying
//! ability — has no catalog primitive (Conjure is deferred engine
//! debt), so it is GAP'd.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Juggle the Performance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{R}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player discards their hand, then conjures a duplicate of each of seven random cards from the library of the player to their right into their hand. The duplicates perpetually gain \"Mana of any type can be spent to cast this spell.\"".into(),
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
    // GAP: "conjures a duplicate of seven random cards ... perpetually gain
    // an ability" — no Conjure primitive (mint cards from another library +
    // perpetual cost-modifying grant). Only the discard-hand clause is emitted.
    vec![Effect::Sequence(
        script::all_players(state)
            .into_iter()
            .map(|p| Effect::Discard {
                player: p,
                count: script::hand_size(state, p),
                choice: DiscardChoice::ControllerChooses,
            })
            .collect(),
    )]
}
