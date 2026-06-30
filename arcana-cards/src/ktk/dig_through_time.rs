//! Dig Through Time — `{6}{U}{U}` instant with Delve. (GAP: the printed effect
//! is "look at the top seven cards, put two into your hand, rest on the bottom"
//! — selection from seven isn't expressible with the single-pick dig effect, so
//! it's modeled as drawing two cards, preserving the two-card advantage a
//! material referee values. Delve is fully engine-wired.)

use arcana_core::effects::{Effect, KeywordAbility};
use arcana_core::mana::ManaCost;
use arcana_core::objects::Characteristics;
use arcana_core::registry::{CardDefinition, CardRegistry, SpellAbilityDef};
use arcana_core::stack::StackEntry;
use arcana_core::state::GameState;
use arcana_core::types::{CardId, ColorSet, TypeLine};

pub fn register(reg: &mut CardRegistry) -> CardId {
    let name = reg.interner_mut().intern("Dig Through Time");
    let chars = Characteristics {
        name,
        mana_cost: Some(ManaCost::parse("{6}{U}{U}").expect("valid cost")),
        colors: ColorSet::blue(),
        types: TypeLine::INSTANT.into(),
        keywords: vec![KeywordAbility::Delve],
        ..Default::default()
    };
    reg.register(
        CardDefinition::new(name, chars).with_spell_ability(SpellAbilityDef {
            text: "Delve. Look at the top seven cards of your library. Put two of them into your hand and the rest on the bottom in a random order.".into(),
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
    vec![Effect::DrawCards { player: entry.controller, count: 2 }]
}
