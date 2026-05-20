//! A-Druidic Ritual — `{2}{G}` sorcery. "You may mill three cards.
//! Then return up to two creature and/or land cards from your
//! graveyard to your hand."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("A-Druidic Ritual");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{2}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "You may mill three cards. Then return up to two creature and/or land cards from your graveyard to your hand.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // "Return up to two creature and/or land cards from your
    // graveyard" needs a disjunctive multi-target graveyard
    // selection, not expressible; emit the mill (the optional "may"
    // is taken).
    vec![Effect::Mill { player: entry.controller, count: 3 }]
}
