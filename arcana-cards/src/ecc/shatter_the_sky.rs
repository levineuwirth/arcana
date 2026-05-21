//! Shatter the Sky — `{2}{W}{W}` sorcery. "Each player who controls a
//! creature with power 4 or greater draws a card. Then destroy all
//! creatures."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::{Characteristics, NULL_OBJECT_ID};
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::targets::ObjectFilter;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Shatter the Sky");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each player who controls a creature with power 4 or \
                       greater draws a card. Then destroy all creatures.".into(),
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
    let mut effects = Vec::new();
    // Each player controlling a power-4+ creature draws a card.
    let big = ObjectFilter::creature().with_min_power(4);
    for p in script::all_players(state) {
        if script::count_matching(state, &big, p) > 0 {
            effects.push(Effect::DrawCards { player: p, count: 1 });
        }
    }
    let ids = script::ids_matching(state, &ObjectFilter::creature(), entry.controller);
    effects.push(Effect::ForEach {
        targets: ids,
        effect: Box::new(Effect::DestroyPermanent { target: NULL_OBJECT_ID }),
    });
    effects
}
