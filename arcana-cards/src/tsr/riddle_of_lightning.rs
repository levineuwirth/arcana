//! Riddle of Lightning — `{3}{R}{R}` instant, "Choose any target. Scry 3, then
//! reveal the top card of your library. Riddle of Lightning deals damage equal
//! to that card's mana value to that permanent or player."
//!
//! GAP: damage amount equal to revealed top card's mana value (runtime library
//! peek + mana-value lookup) not in Effect catalog.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::{TargetRequirement};
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Riddle of Lightning");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{R}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Choose any target. Scry 3, then reveal the top card of your library. Riddle of Lightning deals damage equal to that card's mana value to that permanent or player.".into(),
                target_requirements: vec![TargetRequirement::any_target()],
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
    // Scry 3 is expressible; damage scaled by top card's mana value is not.
    // GAP: damage equal to top library card's mana value not in Effect catalog
    vec![Effect::Scry { player: entry.controller, count: 3 }]
}
