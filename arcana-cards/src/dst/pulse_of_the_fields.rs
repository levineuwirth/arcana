//! Pulse of the Fields — `{1}{W}{W}` instant. "You gain 4 life. Then
//! if an opponent has more life than you, return Pulse of the Fields
//! to its owner's hand." Returning the spell to its owner's hand on
//! resolution isn't expressible — best effort: gain 4 life.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Pulse of the Fields");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You gain 4 life. Then if an opponent has more life than you, return Pulse of the Fields to its owner's hand.".into(),
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
    // GAP: returning a resolving spell from the stack to its owner's hand isn't in the catalog.
    vec![Effect::GainLife { player: entry.controller, amount: 4 }]
}
