//! Cut a Deal — `{2}{W}` sorcery. "Each opponent draws a card, then
//! you draw a card for each opponent who drew a card this way."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Cut a Deal");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{W}").expect("valid cost")),
        colors: ColorSet::white(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Each opponent draws a card, then you draw a card for each opponent who drew a card this way.".into(),
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
    let opponents = script::opponents(state, entry.controller);
    let count = opponents.len() as u32;
    let mut effects: Vec<Effect> = opponents
        .into_iter()
        .map(|p| Effect::DrawCards { player: p, count: 1 })
        .collect();
    effects.push(Effect::DrawCards { player: entry.controller, count });
    effects
}
