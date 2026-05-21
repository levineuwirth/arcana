//! Take Inventory — `{1}{U}` sorcery. Draw a card, then draw cards equal to
//! the number of cards named Take Inventory in your graveyard.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Take Inventory");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Draw a card, then draw cards equal to the number of cards named Take Inventory in your graveyard.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // GAP: cannot filter graveyard cards by name; use generic graveyard count
    // of cards matching the all-permanents filter as a stand-in for the
    // name-matched subset.
    let n = script::graveyard_matching(
        state,
        &ObjectFilter::new(),
        entry.controller,
        entry.controller,
    );
    vec![
        Effect::DrawCards { player: entry.controller, count: 1 },
        Effect::DrawCards { player: entry.controller, count: n },
    ]
}
