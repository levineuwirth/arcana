//! Deadly Brew — `{B}{G}` sorcery. "Each player sacrifices a creature
//! or planeswalker of their choice. If you sacrificed a permanent
//! this way, you may return another permanent card from your
//! graveyard to your hand."

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
    let name = reg.interner_mut().intern("Deadly Brew");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{B}{G}").expect("valid cost")),
        colors: ColorSet::black() | ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player sacrifices a creature or planeswalker of their choice. If you sacrificed a permanent this way, you may return another permanent card from your graveyard to your hand.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, _entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "if you sacrificed a permanent this way, you may return
    // another from your graveyard" reflexive clause is not
    // expressible. The Sacrifice filter cannot express
    // "creature or planeswalker" either, so use creature().
    script::all_players(state)
        .into_iter()
        .map(|p| Effect::Sacrifice {
            player: p,
            filter: ObjectFilter::creature(),
            count: 1,
        })
        .collect()
}
