//! Rhystic Tutor — `{2}{B}` sorcery. "Unless any player pays {2},
//! search your library for a card, put that card into your hand, then
//! shuffle."
//!
//! # GAP
//! "Unless any player pays {2}" buyout condition is not expressible —
//! no conditional tutor variant in the catalog. The unconditional
//! tutor (any card to hand) is emitted as best effort.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{ObjectFilter, TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rhystic Tutor");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Unless any player pays {2}, search your library for a card, put that card into your hand, then shuffle.".into(),
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
    // GAP: "unless any player pays {2}" conditional not expressible; emitted unconditionally
    vec![Effect::TutorToHand {
        player: entry.controller,
        filter: ObjectFilter::permanent(),
        reveal: false,
    }]
}
