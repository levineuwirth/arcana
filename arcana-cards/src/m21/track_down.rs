//! Track Down — `{1}{G}` sorcery. "Scry 3, then reveal the top card
//! of your library. If it's a creature or land card, draw a card."
//! The conditional draw depends on revealing the post-scry top card,
//! which the catalog cannot inspect; we emit Scry 3 and GAP the
//! conditional draw.

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Track Down");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Scry 3, then reveal the top card of your library. If it's a creature or land card, draw a card.".into(),
            target_requirements: vec![],
            modal: None,
            effect: resolve,
        }),
    )
}

fn resolve(_state: &GameState, entry: &StackEntry, _reg: &CardRegistry) -> Vec<Effect> {
    // GAP: "reveal the top card; if it's a creature or land, draw" —
    // no effect to inspect/condition on the revealed top card. Scry 3
    // is emitted.
    vec![Effect::Scry { player: entry.controller, count: 3 }]
}
