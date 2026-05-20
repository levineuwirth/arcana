//! Explosion of Riches — `{5}{R}` sorcery. "Draw a card, then each
//! other player may draw a card. Whenever a card is drawn this way,
//! Explosion of Riches deals 5 damage to target opponent chosen at
//! random from among your opponents."
//!
//! The "may draw" optionality and the per-draw random-target damage
//! trigger aren't expressible; the controller's mandatory draw plus
//! each opponent's draw is emitted (the optional/trigger rider is a
//! GAP).

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::script;
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Explosion of Riches");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{R}").expect("valid cost")),
        colors: ColorSet::red(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Draw a card, then each other player may draw a card. Whenever a card is drawn this way, Explosion of Riches deals 5 damage to target opponent chosen at random from among your opponents.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    let mut effects = vec![Effect::DrawCards { player: entry.controller, count: 1 }];
    for p in script::opponents(state, entry.controller) {
        effects.push(Effect::DrawCards { player: p, count: 1 });
    }
    // GAP: "may draw" optionality and the per-draw random-target
    // 5-damage trigger are not catalog-expressible.
    effects
}
