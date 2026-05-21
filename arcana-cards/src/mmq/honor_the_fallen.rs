//! Honor the Fallen — `{1}{W}` instant. "Exile all creature cards
//! from all graveyards. You gain 1 life for each card exiled this
//! way."

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
    let name = reg.interner_mut().intern("Honor the Fallen");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::INSTANT.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Exile all creature cards from all graveyards. You gain 1 life for each card exiled this way.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // Tally creature cards across all graveyards for the life gain.
    let mut total = 0u32;
    for p in script::all_players(state) {
        total += script::graveyard_matching(
            state,
            &ObjectFilter::creature(),
            p,
            entry.controller,
        );
    }
    // GAP: no primitive to exile cards out of graveyards en masse —
    // only the life-gain rider is emitted (count is computed honestly).
    vec![Effect::GainLife {
        player: entry.controller,
        amount: total,
    }]
}
