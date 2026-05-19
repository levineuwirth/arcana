//! Tainted Indulgence — `{U}{B}` instant. "Draw two cards. Then discard a card unless
//! there are five or more mana values among cards in your graveyard."
//!
//! GAP: conditional discard based on counting distinct mana values in graveyard is not
//! in the catalog. Draw two is expressible; the conditional discard is omitted.

use arcana_core::effects::Effect;
use arcana_core::effects::DiscardChoice;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Tainted Indulgence");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{U}{B}").expect("valid cost")),
        colors: ColorSet::blue() | ColorSet::black(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw two cards. Then discard a card unless there are five or more mana values among cards in your graveyard.".into(),
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
    // GAP: conditional discard based on count of distinct mana values in graveyard not in catalog
    vec![
        Effect::DrawCards { player: entry.controller, count: 2 },
        Effect::Discard { player: entry.controller, count: 1, choice: DiscardChoice::ControllerChooses },
    ]
}
