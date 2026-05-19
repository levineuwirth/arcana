//! Thrilling Discovery — `{R}{W}` sorcery. "You gain 2 life. Then you may
//! discard two cards. If you do, draw three cards."
//
// GAP: "you may discard two cards — if you do, draw three" requires a
// conditional "may" choice by the controller at resolution, which the
// catalog's Conditional effect does not model (no may-discard-then-draw
// conditional). Emitting the gain-life only; the conditional draw/discard
// is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Thrilling Discovery");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}{W}").expect("valid cost")),
        colors: ColorSet::red() | ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You gain 2 life. Then you may discard two cards. If you do, draw three cards.".into(),
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
        Effect::GainLife { player: entry.controller, amount: 2 },
        // GAP: optional "may discard 2, if you do draw 3" conditional choice not expressible
    ]
}
