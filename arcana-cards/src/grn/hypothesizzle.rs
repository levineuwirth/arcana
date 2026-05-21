//! Hypothesizzle — `{3}{U}{R}` instant. "Draw two cards. Then you may
//! discard a nonland card. When you do, Hypothesizzle deals 4 damage
//! to target creature." Optional discard with reflexive trigger is
//! not in catalog; emit the draws and GAP the rider.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Hypothesizzle");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}{R}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::red(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw two cards. Then you may discard a nonland card. When you do, Hypothesizzle deals 4 damage to target creature.".into(),
            target_requirements: vec![TargetRequirement::target_creature()],
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
    // GAP: "you may discard a nonland card. When you do, deal 4 damage" reflexive trigger not in catalog.
    vec![Effect::DrawCards { player: entry.controller, count: 2 }]
}
