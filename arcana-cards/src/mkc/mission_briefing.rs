//! Mission Briefing — `{U}{U}` instant.
//! "Surveil 2, then choose an instant or sorcery card in your graveyard. You may
//! cast it this turn. If that spell would be put into your graveyard, exile it instead."
//!
//! GAP: "choose a card in your graveyard and cast it this turn" — flashback-style
//! graveyard cast with replacement effect is not in the engine catalog.
//! Partial: emit Surveil 2 only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Mission Briefing");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Surveil 2, then choose an instant or sorcery card in your graveyard. You may cast it this turn. If that spell would be put into your graveyard, exile it instead.".into(),
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
    // GAP: graveyard cast with exile-replacement not in catalog
    vec![Effect::Surveil { player: entry.controller, count: 2 }]
}
