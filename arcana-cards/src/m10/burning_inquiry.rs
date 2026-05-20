//! Burning Inquiry — `{R}` sorcery. "Each player draws three cards,
//! then discards three cards at random."

use arcana_core::effects::{DiscardChoice, Effect};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Burning Inquiry");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Each player draws three cards, then discards three cards at random.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, _entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let players = script::all_players(state);
    let mut effects: Vec<Effect> = players
        .iter()
        .map(|p| Effect::DrawCards { player: *p, count: 3 })
        .collect();
    effects.extend(players.iter().map(|p| Effect::Discard {
        player: *p,
        count: 3,
        choice: DiscardChoice::Random,
    }));
    vec![Effect::Sequence(effects)]
}
