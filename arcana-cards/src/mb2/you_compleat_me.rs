//! You Compleat Me — `{1}{B}{B}` sorcery. "If your life total is greater than
//! 10, it becomes 10. For the rest of the game, your maximum life total is 10.
//! You get an emblem with ..."
//!
//! GAP: "maximum life total" cap — no Effect variant for setting a maximum life total.
//! GAP: "you get an emblem" — no Effect variant for creating emblems.
//! Best-effort: emit SetLifeTotal capped at 10 (conditional on life > 10).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("You Compleat Me");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "If your life total is greater than 10, it becomes 10. For the rest of the game, your maximum life total is 10. You get an emblem with \"Pay 2 life: Add one mana of any color\" and \"At the beginning of your upkeep, you draw a card and you lose 1 life.\"".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: "maximum life total" persistent cap — no Effect variant
    // GAP: "you get an emblem" — no Effect variant for emblems
    let life = script::life(state, entry.controller);
    if life > 10 {
        vec![Effect::SetLifeTotal { player: entry.controller, amount: 10 }]
    } else {
        Vec::new()
    }
}
