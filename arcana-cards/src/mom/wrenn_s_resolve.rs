//! Wrenn's Resolve — `{1}{R}` sorcery. "Exile the top two cards of
//! your library. Until the end of your next turn, you may play those
//! cards." A two-card impulse draw.
//!
//! Modeled with [`Effect::ImpulseExile`] (CR 601.3e): the top two
//! library cards are exiled and flagged playable. NOTE/partial: the
//! engine clears the impulse-play flags at the end of THIS turn, not
//! at the end of your NEXT turn — the extra-turn window is a
//! documented fidelity gap in the underlying primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Wrenn's Resolve");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile the top two cards of your library. Until the end of your next turn, you may play those cards.".into(),
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
    vec![Effect::ImpulseExile { player: entry.controller, count: 2 }]
}
