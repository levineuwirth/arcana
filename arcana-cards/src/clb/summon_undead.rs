//! Summon Undead — `{4}{B}` sorcery. "You may mill three cards. Then return a
//! creature card from your graveyard to the battlefield."

use arcana_core::effects::Effect;
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Summon Undead");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{4}{B}").expect("valid cost")),
        colors: ColorSet::black(),
        types: TypeLine::SORCERY.into(),
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars)
            .with_spell_ability(SpellAbilityDef {
                text: "You may mill three cards. Then return a creature card from your graveyard to the battlefield.".into(),
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
    // "You may mill three cards" — treated as the standard mill (the optionality is not modeled).
    // "Return a creature card from your graveyard to the battlefield" is a non-targeted choice;
    // GAP: no untargeted ReturnFromGraveyardToBattlefield primitive (only the targeted variant).
    vec![Effect::Mill { player: entry.controller, count: 3 }]
}
