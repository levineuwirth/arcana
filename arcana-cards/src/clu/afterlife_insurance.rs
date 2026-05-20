//! Afterlife Insurance — `{1}{W/B}` instant. "Creatures you control
//! gain afterlife 1 until end of turn. Draw a card." Granting a
//! keyword to a whole group has no primitive (GrantKeyword is
//! single-target); the draw is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Afterlife Insurance");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W/B}").expect("valid cost")),
        colors: ColorSet::white() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Creatures you control gain afterlife 1 until end of turn. Draw a card.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: granting afterlife 1 to all creatures you control (group
    // keyword grant) has no primitive.
    vec![Effect::DrawCards { player: entry.controller, count: 1 }]
}
