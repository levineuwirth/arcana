//! Mission Briefing — `{U}{U}` instant, "Surveil 2, then choose an instant or
//! sorcery card from your graveyard, you may cast it this turn, if it would
//! be put into your graveyard exile it instead."
//!
//! GAP: graveyard-cast-with-exile-replacement mechanic not expressible.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
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
    vec![
        Effect::Surveil { player: entry.controller, count: 2 },
        // GAP: choose instant/sorcery from graveyard, cast it this turn with
        // exile-instead-of-graveyard replacement effect — not expressible
    ]
}
