//! Ideas Unbound — `{U}{U}` sorcery (Arcane). "Draw three cards.
//! Discard three cards at the beginning of the next end step."
//!
//! # GAP
//! "Discard three cards at the beginning of the next end step" is a
//! delayed triggered effect — not expressible with the immediate
//! `Effect::Discard`. The draw is modeled; the delayed discard is
//! noted.

use arcana_core::effects::{DiscardChoice, Effect};
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
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw three cards. Discard three cards at the beginning of the next end step.".into(),
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
    // GAP: delayed triggered discard at next end step not expressible
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
