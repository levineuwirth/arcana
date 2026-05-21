//! Roots of Wisdom — `{1}{G}` sorcery. "Mill three cards, then return a land
//! card or Elf card from your graveyard to your hand. If you can't, draw a
//! card."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Roots of Wisdom");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{1}{G}").expect("valid cost")),
        colors: ColorSet::green(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "Mill three cards, then return a land card or Elf card from your graveyard to your hand. If you can't, draw a card.".into(),
                target_requirements: vec![],
                modal: None,
                effect: resolve,
            }),
    )
}

fn resolve(
    _state: &GameState,
    entry: &StackEntry,
    _reg: &CardRegistry,
) -> Vec<Effect> {
    // "return a land/Elf card from your graveyard" is a non-targeted graveyard choice with
    // an "if you can't, draw" fallback — no untargeted graveyard-return primitive exists.
    // GAP: the conditional return/draw clause; emit only the mill.
    vec![Effect::Mill { player: entry.controller, count: 3 }]
}
