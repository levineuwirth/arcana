//! Infectious Inquiry — `{2}{B}` sorcery. "You draw two cards and you lose 2
//! life. Each opponent gets a poison counter."
//!
//! GAP: "each opponent gets a poison counter" — no Effect variant for distributing
//! poison counters to players. Draw and life loss are expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Infectious Inquiry");
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
                text: "You draw two cards and you lose 2 life. Each opponent gets a poison counter.".into(),
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
    // GAP: "each opponent gets a poison counter" — no Effect variant for player poison counters
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::LoseLife { player: entry.controller, amount: 2 },
    ]
}
