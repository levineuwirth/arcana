//! Hoarder's Greed — `{3}{B}` sorcery. "You lose 2 life and draw two
//! cards, then clash with an opponent. If you win, repeat this
//! process."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hoarder's Greed");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You lose 2 life and draw two cards, then clash with an \
                       opponent. If you win, repeat this process.".into(),
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
    // GAP: "clash" and the "if you win, repeat" loop have no Effect;
    // emit the one-time life loss and draw.
    vec![
        Effect::LoseLife { player: entry.controller, amount: 2 },
        Effect::DrawCards { player: entry.controller, count: 2 },
    ]
}
