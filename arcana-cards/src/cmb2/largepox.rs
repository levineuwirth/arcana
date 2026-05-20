//! Largepox — `{B}{B}{B}{B}` sorcery. "Each player discards a card,
//! then loses 1 life, then sacrifices [permanents], then exiles a card
//! from their graveyard, mills one, removes a counter, gets a poison
//! counter."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Largepox");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{B}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player discards a card, then loses 1 life, then sacrifices an artifact, a creature, an enchantment, a land, a planeswalker, and a tribal permanent, then exiles a card from their graveyard, then puts the top card of their library into their graveyard, then removes a counter from a permanent they control, then gets a poison counter.".into(),
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
    // Partial: per-player discard / lose-life / sacrifice-a-creature /
    // mill are expressible. The remaining clauses (sacrifice each of
    // the other permanent types, exile from graveyard, remove a
    // counter, poison counter) have no per-player catalog effect and
    // are omitted.
    let mut effects = Vec::new();
    for p in script::all_players(state) {
        effects.push(Effect::Discard {
            player: p,
            count: 1,
            choice: DiscardChoice::ControllerChooses,
        });
        effects.push(Effect::LoseLife { player: p, amount: 1 });
        effects.push(Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: 1,
        });
        effects.push(Effect::Mill { player: p, count: 1 });
    }
    vec![Effect::Sequence(effects)]
}
