//! Dusk's Landing — `{B}` sorcery.
//! "Draw a card. If an opponent lost life this turn and you gained life
//! this turn, seek two Vampire cards instead."
//
// Keywords (Scryfall-parsed): Seek
//
// GAP: Seek mechanic not supported.
// GAP: conditional check "opponent lost life this turn AND you gained
//      life this turn" not expressible (no Effect::Conditional with
//      life-event history).
// Best effort: draw 1 card.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dusk's Landing");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card. If an opponent lost life this turn and you gained life this turn, seek two Vampire cards instead.".into(),
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
    // GAP: Seek mechanic; GAP: conditional check on life-event history
    vec![Effect::DrawCards { player: entry.controller, count: 1 }]
}
