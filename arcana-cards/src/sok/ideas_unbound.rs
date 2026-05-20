//! Ideas Unbound — `{U}{U}` sorcery — Arcane. "Draw three cards.
//! Discard three cards at the beginning of the next end step."
//!
//! DelayedAction supports Sacrifice/Exile/ReturnToHand on a known
//! permanent id — not a player-discards-N at end step. The delayed
//! discard is GAP'd; only the draw is modeled.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Ideas Unbound");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw three cards. Discard three cards at the beginning of the next end step.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: end-step-delayed discard-N on a player has no DelayedAction variant.
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
