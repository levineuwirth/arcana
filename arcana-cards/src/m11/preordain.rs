//! Preordain — `{U}` sorcery. "Scry 2, then draw a card." The seed's
//! touchstone for sequential multi-effect resolution: the Scry pushes
//! an `OrderCards` prompt mid-resolution, the engine parks the
//! remaining draw into [`PendingResolution`], and the draw runs on
//! resume.
//!
//! # Rules references
//!
//! * CR 608.2 — resolution sequence. Effects in a spell's rules text
//!   are executed top-to-bottom, finishing each before starting the
//!   next. Scry's card-order choice is a part of its effect, so the
//!   draw doesn't begin until the player submits placements.
//! * CR 701.19 — Scry. "Look at top N of library, put any number on
//!   the bottom (in any order), the rest on top (in any order)."
//!   Engine wiring: [`Effect::Scry`] pushes
//!   [`ChoiceKind::OrderCards`] with `TopOfLibrary` and
//!   `BottomOfLibrary` destinations.
//!
//! # Implementation shape
//!
//! Resolver returns `vec![Effect::Scry {count:2}, Effect::DrawCards
//! {count:1}]`. The engine's `execute_effects_or_park`
//! (engine.rs:2260) iterates the vec, detects the scry's pushed
//! choice, parks the pending draw, and resumes it on choice submit.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Preordain");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Scry 2, then draw a card.".into(),
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
    vec![
        Effect::Scry { player: entry.controller, count: 2 },
        Effect::DrawCards { player: entry.controller, count: 1 },
    ]
}
