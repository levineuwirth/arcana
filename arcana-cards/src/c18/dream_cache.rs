//! Dream Cache — `{2}{U}` sorcery, "Draw three cards, then put two cards from your hand
//! on top of your library or on the bottom of your library (your choice, but they must
//! both go to the same zone)."
//!
//! GAP: Put exactly 2 cards from hand simultaneously to top or bottom (choice; both to same zone;
//! no Effect variant for hand-to-library placement).

use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::TargetRequirement;
use arcana_core::types::{CardId, ColorSet, TypeLine};
use arcana_core::effects::Effect;

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dream Cache");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw three cards, then put two cards from your hand on top of your library or on the bottom of your library.".into(),
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
    // GAP: put 2 cards from hand to top or bottom of library (no hand-to-library placement Effect)
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
