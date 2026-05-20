//! Rejoin the Fight — `{5}{B}` sorcery. "Mill three cards. Then
//! starting with the next opponent in turn order, each opponent
//! chooses a creature card in your graveyard that hasn't been chosen.
//! Return each card chosen this way to the battlefield under your
//! control." The opponent-chooses-from-your-graveyard reanimation has
//! no primitive; the mill is emitted.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Rejoin the Fight");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{5}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Mill three cards. Then starting with the next opponent in turn order, each opponent chooses a creature card in your graveyard that hasn't been chosen. Return each card chosen this way to the battlefield under your control.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: opponent-chooses-card-from-your-graveyard reanimation has no
    // primitive (no targeted card-in-graveyard chooser per opponent).
    vec![Effect::Mill { player: entry.controller, count: 3 }]
}
