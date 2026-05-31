//! Rite of Flame — `{R}` sorcery. "Add {R}{R}, then add {R} for each
//! card named Rite of Flame in each graveyard."
//!
//! Ritual: the base {R}{R} is fixed; the extra {R}s are dynamic —
//! one per card named "Rite of Flame" across every player's graveyard.
//! Computed via `script::graveyard_matching` summed over all players
//! with a name-filtered `ObjectFilter`.

use arcana_core::effects::Effect;
use arcana_core::mana::{ManaCost, ManaUnit};
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, ManaColor, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rite of Flame");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Add {R}{R}, then add {R} for each card named Rite of Flame in each graveyard.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(
    state: &GameState,
    entry: &StackEntry,
    reg: &CardRegistry,
) -> Vec<Effect> {
    let name = reg.interner().lookup("Rite of Flame");
    let filter = ObjectFilter { name, ..ObjectFilter::default() };
    let extra: u32 = script::all_players(state)
        .into_iter()
        .map(|p| script::graveyard_matching(state, &filter, p, entry.controller))
        .sum();
    let total = 2 + extra;
    vec![Effect::AddMana {
        player: entry.controller,
        mana: vec![ManaUnit::plain(ManaColor::Red, entry.source); total as usize],
    }]
}
