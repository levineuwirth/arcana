//! Lat-Nam's Legacy — `{1}{U}` instant. "Shuffle a card from your hand into
//! your library. If you do, draw two cards at the beginning of the next turn's
//! upkeep."
//! GAP: Shuffle-a-hand-card-into-library choice and delayed draw (next
//! upkeep trigger) are not in the Effect catalog. Best effort: draw two cards
//! immediately (drop the conditional delayed trigger).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Lat-Nam's Legacy");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Shuffle a card from your hand into your library. If you do, draw two cards at the beginning of the next turn's upkeep.".into(),
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
    // GAP: shuffle-hand-card-into-library and delayed upkeep draw not in Effect catalog
    vec![Effect::DrawCards { player: entry.controller, count: 2 }]
}
