//! Teachings of the Archaics — `{2}{U}` sorcery — Lesson. "If an
//! opponent has more cards in hand than you, draw two cards. Draw three
//! cards instead if an opponent has at least four more cards in hand
//! than you."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Teachings of the Archaics");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "If an opponent has more cards in hand than you, draw two cards. Draw three cards instead if an opponent has at least four more cards in hand than you.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mine = script::hand_size(state, entry.controller) as i64;
    let best = script::opponents(state, entry.controller)
        .into_iter()
        .map(|p| script::hand_size(state, p) as i64)
        .max()
        .unwrap_or(0);
    let diff = best - mine;
    let count: u32 = if diff >= 4 {
        3
    } else if diff >= 1 {
        2
    } else {
        0
    };
    if count == 0 {
        return Vec::new();
    }
    vec![Effect::DrawCards {
        player: entry.controller,
        count,
    }]
}
