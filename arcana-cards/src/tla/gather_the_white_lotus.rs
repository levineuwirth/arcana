//! Gather the White Lotus — `{4}{W}` sorcery.
//! "Create a 1/1 white Ally creature token for each Plains you control.
//! Scry 2."
//!
//! # GAP: token count = number of Plains controller controls
//! The engine has no mechanism to count permanents of a given subtype at
//! resolve time and produce that many `Effect::CreateToken` entries. The
//! Scry 2 rider is expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Gather the White Lotus");
    let _ally = reg.interner_mut().intern("Ally");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Create a 1/1 white Ally creature token for each Plains you control. Scry 2.".into(),
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
    // GAP: create N tokens where N = number of Plains controller controls
    vec![Effect::Scry { player: entry.controller, count: 2 }]
}
