//! Reign of the Pit — `{4}{B}{B}` sorcery. "Each player sacrifices a
//! creature of their choice. Create an X/X black Demon creature token
//! with flying, where X is the total power of the creatures sacrificed
//! this way."

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
    let name = reg.interner_mut().intern("Reign of the Pit");
    let _demon = reg.interner_mut().intern("Demon");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player sacrifices a creature of their choice. Create an X/X black Demon creature token with flying, where X is the total power of the creatures sacrificed this way.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, _entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: the token's X/X is the total power of the creatures
    // sacrificed *this way*, which cannot be measured after the
    // sacrifices resolve. Emitting only the each-player sacrifices.
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: 1,
        })
        .collect()
}
