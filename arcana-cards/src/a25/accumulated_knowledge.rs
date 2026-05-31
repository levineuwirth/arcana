//! Accumulated Knowledge — `{1}{U}` instant. "Draw a card, then draw
//! cards equal to the number of cards named Accumulated Knowledge in
//! all graveyards." The second draw count is dynamic: it sums, over
//! every player's graveyard, the cards whose name matches this one.

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
    let name = reg.interner_mut().intern("Accumulated Knowledge");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw a card, then draw cards equal to the number of cards named \
                   Accumulated Knowledge in all graveyards."
                .into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, reg: &CardRegistry) -> Vec<Effect> {
    let nm = reg.interner().lookup("Accumulated Knowledge");
    let filter = ObjectFilter {
        name: nm,
        ..ObjectFilter::default()
    };
    let extra: u32 = script::all_players(state)
        .into_iter()
        .map(|p| script::graveyard_matching(state, &filter, p, entry.controller))
        .sum();
    vec![
        Effect::DrawCards {
            player: entry.controller,
            count: 1,
        },
        Effect::DrawCards {
            player: entry.controller,
            count: extra,
        },
    ]
}
