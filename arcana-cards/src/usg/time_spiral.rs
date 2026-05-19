//! Time Spiral — `{4}{U}{U}` sorcery. "Exile Time Spiral. Each player shuffles
//! their hand and graveyard into their library, then draws seven cards. You
//! untap up to six lands."
//!
//! GAP: 'each player shuffles hand and graveyard into library' and 'untap up
//! to six lands of your choice' are not expressible with available Effects.
//! Best effort: draw 7 for the controller only.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Time Spiral");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Exile Time Spiral. Each player shuffles their hand and graveyard into their library, then draws seven cards. You untap up to six lands.".into(),
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
    // GAP: exile self (spell on stack), each player shuffles hand+graveyard
    //      into library, untap up to 6 chosen lands
    vec![
        Effect::DrawCards { player: entry.controller, count: 7 },
    ]
}
