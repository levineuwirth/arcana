//! Urza's Guilt — `{2}{U}{B}` sorcery. "Each player draws two cards, then
//! discards three cards, then loses 4 life."
//!
//! # GAP: iterating over all players requires ForEachPlayer not in catalog.
//! Best-effort: emit for controller only; full multi-player is a gap.

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Urza's Guilt");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player draws two cards, then discards three cards, then loses 4 life.".into(),
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
    // GAP: no ForEachPlayer — controller only approximation
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::Discard { player: entry.controller, count: 3, choice: DiscardChoice::ControllerChooses },
        Effect::LoseLife { player: entry.controller, amount: 4 },
    ]
}
