//! Sphinx's Revelation — `{X}{W}{U}{U}` instant. "You gain X life and draw X
//! cards." Reads the cast's announced X (`entry.x_value`).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Sphinx's Revelation");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{X}{W}{U}{U}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You gain X life and draw X cards.".into(),
            target_requirements: Vec::new(),
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
    let x = entry.x_value.unwrap_or(0);
    if x == 0 {
        return Vec::new();
    }
    let p = entry.controller;
    vec![
        Effect::GainLife { player: p, amount: x },
        Effect::DrawCards { player: p, count: x },
    ]
}
