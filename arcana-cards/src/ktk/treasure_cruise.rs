//! Treasure Cruise — `{7}{U}` sorcery with Delve. "Draw three cards." Delve is
//! fully engine-wired (cast-time graveyard-exile generic reduction via the
//! Delve keyword); the card just declares it and draws.

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Treasure Cruise");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{7}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::SORCERY.into(),
        keywords: vec![KeywordAbility::Delve],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Delve. Draw three cards.".into(),
            target_requirements: Vec::new(),
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
    vec![Effect::DrawCards { player: entry.controller, count: 3 }]
}
