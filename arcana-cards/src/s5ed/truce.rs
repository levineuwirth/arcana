//! Truce — `{2}{W}` instant. "Each player may draw up to two cards. For each card less
//! than two a player draws this way, that player gains 2 life."
//!
//! GAP: "may draw up to N" player-choice draw with life gain for each card not drawn
//! (no optional/variable draw with conditional life gain per player).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Truce");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player may draw up to two cards. For each card less than two a player draws this way, that player gains 2 life.".into(),
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
    // GAP: "may draw up to 2" per player with life gain for each card not drawn
    vec![Effect::DrawCards { player: entry.controller, count: 2 }]
}
