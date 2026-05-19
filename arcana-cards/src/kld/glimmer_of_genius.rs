//! Glimmer of Genius — `{3}{U}` instant. "Scry 2, then draw two cards.
//! You get {E}{E} (two energy counters)."
//!
//! GAP: Energy counters ({E}) are not in the engine Effect catalog.
//! The Scry and DrawCards effects are rendered; the energy gain is not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Glimmer of Genius");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{3}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Scry 2, then draw two cards. You get {E}{E}.".into(),
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
    // GAP: Effect::GainEnergy (energy counters) is not in the catalog.
    vec![
        Effect::Scry { player: entry.controller, count: 2 },
        Effect::DrawCards { player: entry.controller, count: 2 },
    ]
}
