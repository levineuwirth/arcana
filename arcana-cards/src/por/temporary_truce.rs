//! Temporary Truce — `{1}{W}` sorcery. "Each player may draw up to two cards.
//! For each card less than two a player draws this way, that player gains 2
//! life."
//!
//! GAP: 'each player may draw up to N' with variable amounts and conditional
//! life gain per card-less-than-two is not expressible. Best effort: controller
//! draws 2.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Temporary Truce");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
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
    // GAP: 'each player may draw up to 2' choice + conditional life gain
    vec![Effect::DrawCards { player: entry.controller, count: 2 }]
}
