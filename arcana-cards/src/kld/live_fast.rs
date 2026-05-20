//! Live Fast — `{2}{B}` sorcery. "You draw two cards, lose 2 life,
//! and get {E}{E} (two energy counters)."
//!
//! Draw and life loss are expressed; energy counters have no
//! primitive.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Live Fast");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You draw two cards, lose 2 life, and get {E}{E} (two energy counters).".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "get {E}{E}" — no energy-counter primitive.
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::LoseLife { player: entry.controller, amount: 2 },
    ]
}
